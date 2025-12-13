# Frontend Architecture Guidelines

This document describes the layered architecture design principles for the Garage UI Frontend, based on Clean Architecture and DDD (Domain-Driven Design) concepts.

> **Note**: This project uses a Monorepo structure where the Rust Server and Frontend coexist. To maintain consistency, all naming conventions are unified using `snake_case`.

---

## 📁 Directory Structure

```
frontend/
├── client/
│   └── app/
│       ├── presentation/          # Presentation Layer - UI related
│       │   ├── layout/            # Global Layout Components
│       │   ├── pages/             # Pages (feature-based)
│       │   │   └── {feature}/
│       │   │       ├── components/    # Feature-specific components
│       │   │       ├── list/          # List page
│       │   │       ├── detail/        # Detail page
│       │   │       └── create/        # Create page
│       │   └── shared/            # Shared UI components
│       │
│       ├── application/           # Application Layer - Use Case Coordination
│       │   ├── commands/          # Write Operations (CQRS Command)
│       │   ├── queries/           # Read Operations (CQRS Query)
│       │   ├── stores/            # State Management (Feature Store)
│       │   └── states/            # State Containers (Signal State)
│       │
│       ├── domain/                # Domain Layer - Business Logic
│       │   ├── aggregates/        # Domain Aggregates
│       │   └── value_objects/     # Value Objects
│       │
│       └── infrastructure/        # Infrastructure Layer - Technical Implementation
│           ├── api/               # API Services (HTTP Calls)
│           ├── http/              # HTTP Client Abstraction
│           ├── repositories/      # Data Access Layer (Entity → Aggregate Conversion)
│           ├── navigation/        # Routing/Navigation Services
│           └── i18n/              # I18n Services
│
└── shared/                        # Frontend + BFF Shared
    ├── contracts/                 # Data Transfer Contracts (DTO + Validation)
    ├── entity/                    # API Response Entities
    └── utility/                   # Utility Functions
```

---

## 🏗️ Layered Architecture

### Dependency Rule (Clean Architecture)

```
                    ┌─────────────────────────┐
                    │    Presentation Layer   │
                    │   (Component, Facade)   │
                    └───────────┬─────────────┘
                                │ Depends on ↓
                    ┌───────────▼─────────────┐
                    │    Application Layer    │
                    │ (Store, Query, Command) │
                    └───────────┬─────────────┘
                                │ Depends on ↓
┌───────────────────────────────▼───────────────────────────────┐
│                        Domain Layer                            │
│                   (Aggregate, Value Object)                    │
│                         【Core Layer】                          │
└───────────────────────────────▲───────────────────────────────┘
                                │ Depends on ↑ (Dependency Inversion)
                    ┌───────────┴─────────────┐
                    │   Infrastructure Layer  │
                    │ (Repository, API, HTTP) │
                    └─────────────────────────┘
```

**Key Principles:**

- **Domain Layer is Core**: Does not depend on any external frameworks or services.
- **Dependency Inversion**: Infrastructure depends on Domain (via Aggregate conversion).
- **Layers depend downwards**: Presentation → Application → Domain.
- **Infrastructure depends inwards**: Repository converts Entity to Aggregate.

---

## 📦 Layer Responsibilities

### 1. Presentation Layer

#### Component

- **Responsibilities**: Pure UI rendering, event binding, template logic.
- **Should NOT have**: Business logic, direct API calls, complex state management.
- **Dependencies**: Store (Read state), Facade (Execute operations).

```typescript
@Component({ ... })
export class AccessKeyListComponent {
  readonly store = inject(AccessKeyStore);
  readonly facade = inject(AccessKeyListFacade);

  open_detail(key: AccessKeyListItemAggregate): void {
    this.facade.open_detail(key.id);
  }
}
```

#### Facade

- **Responsibilities**: Coordinates page-level logic, composes multiple Application services.
- **Scope**: `@Injectable()` without providedIn (Page-level lifecycle, destroyed with Component).
- **Dependencies**: Store, Query, Command, Infrastructure Services.

**Facade Responsibility Classification:**

| Responsibility Category | Description | Example Methods |
| --------------- | ---------------------- | ---------------------------------------------- |
| **Data Loading** | Call Query to fetch data | `load_data()`, `load_more()` |
| **Navigation** | Route transitions | `open_detail()`, `create_key()` |
| **User Interaction** | Confirm dialogs, notifications | `confirm_delete()` |
| **Page State** | Page-level UI state | `enter_selection_mode()`, `cancel_selection()` |
| **Layout Config** | Set page title, action buttons | `set_default_actions()` |

```typescript
@Injectable()
export class AccessKeyListFacade {
  private readonly store = inject(AccessKeyStore);
  private readonly list_query = inject(ListAccessKeysQuery);
  private readonly delete_command = inject(DeleteAccessKeysCommand);
  private readonly navigation = inject(NavigationService);
  private readonly confirm_dialog = inject(ConfirmDialogService);

  // Page-level UI State (Non-domain state)
  readonly selection_mode = signal(false);

  async load_data(page: number, search?: string): Promise<void> {
    this.store.set_loading(true);
    try {
      const result = await this.list_query.execute({ page, page_size: 20, search });
      this.store.set_items(result.rows, page);
      this.store.set_total(result.total);
    } finally {
      this.store.set_loading(false);
    }
  }

  open_detail(id: string): void {
    this.navigation.to_access_key_detail(id);
  }

  async confirm_delete(): Promise<void> {
    const ids = this.store.selected_ids();
    const confirmed = await this.confirm_dialog.confirm_delete('access_key', ids.length);
    if (confirmed) {
      await this.delete_command.execute(ids);
    }
  }
}
```

---

### 2. Application Layer

#### State (Signal State Container)

- **Responsibilities**: Pure state container using Angular Signals.
- **Scope**: `providedIn: 'root'`.
- **Design Concept**: Separates state from logic, avoiding Store bloat.
- **Should NOT have**: Business logic, API calls, side effects.

**State Extended Responsibilities:**

| Responsibility Category | Description | Example |
| ----------------- | ------------------------------ | ------------------------------------------- |
| **Computed** | Derived state calculation | `has_selection`, `is_valid`, `display_name` |
| **reset()** | Reset state to initial values | Clean up when leaving page |
| **snapshot()** | Save current state snapshot | Save original values for edit cancellation |
| **Validation** | Basic validation during state setting | Ensure data matches expected format |
| **Derived State** | State derived from multiple Signals | `filtered_items`, `sorted_items` |

```typescript
@Injectable({ providedIn: 'root' })
export class AccessKeyState {
  // ========================================
  // Domain State - Uses Aggregate
  // ========================================
  readonly items = signal<AccessKeyListItemAggregate[]>([]);
  readonly selected_item = signal<AccessKeyAggregate | null>(null);
  readonly selected_ids = signal<string[]>([]);

  // ========================================
  // UI State
  // ========================================
  readonly total = signal(0);
  readonly page = signal(1);
  readonly loading = signal(false);
  readonly error = signal<string | null>(null);

  // ========================================
  // Computed Signals
  // ========================================
  readonly has_selection = computed(() => this.selected_ids().length > 0);
  readonly selection_count = computed(() => this.selected_ids().length);
  readonly all_selected = computed(
    () => this.items().length > 0 && this.selected_ids().length === this.items().length,
  );
  readonly is_empty = computed(() => this.items().length === 0 && !this.loading());

  // ========================================
  // Snapshot Support
  // ========================================
  private _snapshot: AccessKeyAggregate | null = null;

  save_snapshot(): void {
    this._snapshot = this.selected_item();
  }

  restore_snapshot(): void {
    if (this._snapshot) {
      this.selected_item.set(this._snapshot);
    }
  }

  clear_snapshot(): void {
    this._snapshot = null;
  }

  // ========================================
  // Reset
  // ========================================
  reset(): void {
    this.items.set([]);
    this.selected_item.set(null);
    this.selected_ids.set([]);
    this.total.set(0);
    this.page.set(1);
    this.loading.set(false);
    this.error.set(null);
    this._snapshot = null;
  }
}
```

#### Store (Feature Store)

- **Responsibilities**: State management entry point, exposes State to external world, provides state mutation methods.
- **Scope**: `providedIn: 'root'` (Global Singleton).
- **Dependencies**: State, Repository.
- **Design Concept**: Store manipulates State, does not directly hold Signals.

```typescript
@Injectable({ providedIn: 'root' })
export class AccessKeyStore {
  private readonly state = inject(AccessKeyState);
  private readonly repository = inject(AccessKeyRepository);

  // ========================================
  // Read-only State Accessors (Exposed to external)
  // ========================================
  readonly items = this.state.items;
  readonly selected_item = this.state.selected_item;
  readonly loading = this.state.loading;
  readonly error = this.state.error;
  readonly has_selection = this.state.has_selection;

  // ========================================
  // State Mutators (Used by Facade)
  // ========================================
  set_loading(value: boolean): void {
    this.state.loading.set(value);
  }

  set_items(items: AccessKeyListItemAggregate[], page: number): void {
    if (page === 1) {
      this.state.items.set(items);
    } else {
      this.state.items.update((current) => [...current, ...items]);
    }
    this.state.page.set(page);
  }

  set_total(total: number): void {
    this.state.total.set(total);
  }

  set_error(error: string | null): void {
    this.state.error.set(error);
  }

  // ========================================
  // Complex Operations (Requires Repository)
  // ========================================
  async load(id: string): Promise<void> {
    this.state.loading.set(true);
    try {
      const aggregate = await this.repository.find_by_id(id);
      this.state.selected_item.set(aggregate);
    } catch (e) {
      this.state.error.set(e instanceof Error ? e.message : 'Failed to load');
    } finally {
      this.state.loading.set(false);
    }
  }

  async delete(ids: string[]): Promise<boolean> {
    this.state.loading.set(true);
    try {
      await this.repository.delete(ids);
      this.state.items.update((items) => items.filter((item) => !ids.includes(item.id)));
      this.clear_selection();
      return true;
    } catch (e) {
      this.state.error.set(e instanceof Error ? e.message : 'Failed to delete');
      return false;
    } finally {
      this.state.loading.set(false);
    }
  }

  // ========================================
  // Selection Operations
  // ========================================
  toggle_select(id: string): void {
    /* ... */
  }
  select_all(): void {
    /* ... */
  }
  clear_selection(): void {
    /* ... */
  }
}
```

#### Query (CQRS Query)

- **Responsibilities**: Read operations, fetches data from Repository and **returns results**.
- **Scope**: `providedIn: 'root'`.
- **Design Concept**: Query strictly retrieves data; the caller (Facade) decides how to handle the results.

**Query Extended Responsibilities (Optional):**

| Responsibility Category | Description | Example |
| ----------------- | -------------------------- | --------------------------------- |
| **Param Processing** | Pre-processing, defaults, normalization | Trim search text, default page size |
| **Caching** | Avoid duplicate requests | Return cached data if not expired |
| **Permission Filter** | Filter based on user permissions | Filter out unauthorized items |
| **Result Transform** | Additional result processing | Sorting, grouping, stats calculation |
| **Multi-Repo** | Compose data from multiple Repositories | Sort Key data and related Bucket data |
| **Validation** | Parameter validation (optional) | Ensure required params exist |

```typescript
@Injectable({ providedIn: 'root' })
export class ListAccessKeysQuery {
  private readonly repository = inject(AccessKeyRepository);

  // Basic Usage: Return Repository result directly
  async execute(params: ListAccessKeyContract): Promise<ListResponse<AccessKeyListItemAggregate>> {
    return await this.repository.find_all(params);
  }
}

// Advanced Usage: Query with Caching
@Injectable({ providedIn: 'root' })
export class GetAccessKeyQuery {
  private readonly repository = inject(AccessKeyRepository);
  private readonly cache = new Map<string, { data: AccessKeyAggregate; timestamp: number }>();
  private readonly CACHE_TTL = 5 * 60 * 1000; // 5 minutes

  async execute(id: string, force_refresh = false): Promise<AccessKeyAggregate> {
    // Check cache
    if (!force_refresh) {
      const cached = this.cache.get(id);
      if (cached && Date.now() - cached.timestamp < this.CACHE_TTL) {
        return cached.data;
      }
    }

    // Fetch from Repository
    const result = await this.repository.find_by_id(id);

    // Update cache
    this.cache.set(id, { data: result, timestamp: Date.now() });

    return result;
  }

  invalidate(id: string): void {
    this.cache.delete(id);
  }

  invalidate_all(): void {
    this.cache.clear();
  }
}

// Advanced Usage: Query composing multiple Repositories
@Injectable({ providedIn: 'root' })
export class GetAccessKeyWithBucketsQuery {
  private readonly key_repo = inject(AccessKeyRepository);
  private readonly bucket_repo = inject(BucketRepository);

  async execute(id: string): Promise<AccessKeyWithBucketsResult> {
    const [key, buckets] = await Promise.all([
      this.key_repo.find_by_id(id),
      this.bucket_repo.find_by_key_id(id),
    ]);

    return { key, buckets };
  }
}
```

#### Command (CQRS Command)

- **Responsibilities**: Write operations, executes changes via Repository and **returns results**.
- **Scope**: `providedIn: 'root'`.

```typescript
@Injectable({ providedIn: 'root' })
export class DeleteAccessKeysCommand {
  private readonly repository = inject(AccessKeyRepository);

  async execute(ids: string[]): Promise<string[]> {
    return await this.repository.delete(ids);
  }
}
```

---

### 3. Domain Layer

#### Aggregate (Domain Aggregate)

- **Responsibilities**: Encapsulates business logic, validation rules, computed properties.
- **Should NOT have**: Angular dependencies, API calls.
- **Characteristics**: Pure TypeScript class, testable.
- **Design Concept**: All data fetched from Repository is converted to Aggregate.

```typescript
// List Item Aggregate (Lightweight)
export class AccessKeyListItemAggregate {
  private constructor(private readonly entity: AccessKeyListItem) {}

  static from(entity: AccessKeyListItem): AccessKeyListItemAggregate {
    return new AccessKeyListItemAggregate(entity);
  }

  get id(): string {
    return this.entity.id;
  }
  get name(): string {
    return this.entity.name;
  }
  get created(): string | undefined {
    return this.entity.created;
  }

  // Computed properties
  get display_name(): string {
    return this.name || this.id.substring(0, 8);
  }

  get is_expired(): boolean {
    if (!this.entity.expiration) return false;
    return new Date(this.entity.expiration) < new Date();
  }
}

// Detail Aggregate (Complete)
export class AccessKeyAggregate {
  private constructor(private readonly entity: AccessKey) {}

  static from(entity: AccessKey): AccessKeyAggregate {
    return new AccessKeyAggregate(entity);
  }

  get id(): string {
    return this.entity.id;
  }
  get name(): string {
    return this.entity.name;
  }
  get permissions(): AccessKeyPermissions {
    return this.entity.permissions;
  }
  get buckets(): AccessKeyBucket[] {
    return this.entity.buckets;
  }

  // Business Logic
  get allow_create_bucket(): boolean {
    return this.permissions.owner || this.permissions.write;
  }

  get bucket_count(): number {
    return this.buckets.length;
  }

  // Validation Rules
  validate_name(name: string): ValidationResult {
    if (!name || name.trim().length === 0) {
      return { is_valid: false, error: 'Name is required' };
    }
    if (name.length > 100) {
      return { is_valid: false, error: 'Name must be at most 100 characters' };
    }
    return { is_valid: true };
  }

  // Convert to Update Contract
  to_update_contract(changes: Partial<AccessKey>): UpdateAccessKeyContract {
    return {
      id: this.id,
      name: changes.name ?? this.name,
      // ...
    };
  }
}
```

---

### 4. Infrastructure Layer

#### Repository

- **Responsibilities**: Data access abstraction, **Entity → Aggregate Conversion**.
- **Scope**: `providedIn: 'root'`.
- **Dependencies**: API Service, Domain Aggregates.
- **Design Concept**: Isolates API implementation, unified return of Aggregates.

> ⚠️ **Guideline**: Repository **must** return Aggregate, never Entity.
> This ensures independency of the Domain Layer; upper layers never need to know the raw API response format.

**Repository Responsibilities:**

| Responsibility Category | Description | Necessity |
| ---------------------- | ---------------------------- | ------ |
| **Entity → Aggregate** | Convert API Entity to Aggregate | Required |
| **API Call** | Access data via API Service | Required |
| **Error Conversion** | Convert API errors to Domain errors | Optional |
| **Data Aggregation** | Compose results from multiple API calls | Optional |

```typescript
@Injectable({ providedIn: 'root' })
export class AccessKeyRepository {
  private readonly api = inject(AccessKeyService);

  async find_all(params: ListAccessKeyContract): Promise<ListResponse<AccessKeyListItemAggregate>> {
    const response = await this.api.list_keys(params.page, params.page_size, params.search);
    return {
      rows: response.rows.map((entity) => AccessKeyListItemAggregate.from(entity)),
      total: response.total,
    };
  }

  async find_by_id(id: string): Promise<AccessKeyAggregate> {
    const entity = await this.api.get_key(id);
    return AccessKeyAggregate.from(entity);
  }

  async create(data: CreateAccessKeyContract): Promise<AccessKeyAggregate> {
    const entity = await this.api.create_key(data);
    return AccessKeyAggregate.from(entity);
  }

  async update(data: UpdateAccessKeyContract): Promise<AccessKeyAggregate> {
    const entity = await this.api.update_key(data);
    return AccessKeyAggregate.from(entity);
  }

  async delete(ids: string[]): Promise<string[]> {
    return await this.api.delete(ids);
  }
}
```

#### API Service

- **Responsibilities**: Pure HTTP call implementation, returns raw Entity.
- **Scope**: `providedIn: 'root'`.
- **Dependencies**: HTTP Client.

```typescript
@Injectable({ providedIn: 'root' })
export class AccessKeyService {
  private readonly http = inject<HttpClient>(HTTP_CLIENT);

  list_keys(
    page: number,
    page_size: number,
    search?: string,
  ): Promise<ListResponse<AccessKeyListItem>> {
    const params = new URLSearchParams({ page: String(page), page_size: String(page_size) });
    if (search) params.set('search', search);
    return this.http.get(KeyPath.BASE, { params });
  }

  get_key(id: string): Promise<AccessKey> {
    return this.http.get(build_path(KeyPath.DETAIL, { id }));
  }

  create_key(request: CreateAccessKeyContract): Promise<AccessKey> {
    return this.http.post(KeyPath.BASE, request);
  }

  update_key(request: UpdateAccessKeyContract): Promise<AccessKey> {
    return this.http.put(build_path(KeyPath.DETAIL, { id: request.id }), request);
  }

  delete(ids: string[]): Promise<string[]> {
    return this.http.post(KeyPath.DELETE, { id: ids });
  }
}
```

---

## 🔄 DI Diagram

Example using `AccessKeyListComponent`:

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                           AccessKeyListComponent                                        │
│                              (Presentation)                                             │
└───────────────┬───────────────────────────────────┬─────────────────────────────────────┘
                │ inject                            │ inject
┌───────────────▼───────────────┐   ┌───────────────▼─────────────────────────────────────┐
│       AccessKeyStore          │   │             AccessKeyListFacade                     │
│       (Application)           │   │         (Presentation - page scope)                 │
└───────────────┬───────────────┘   └───────────────┬─────────────────────────────────────┘
                │ inject                            │ inject
                │                                   │
┌───────────────▼───────────────┐   ┌───────────────┼───────────────────┬─────────────────┐
│       AccessKeyState          │   │               │                   │                 │
│       (Application)           │   ▼               ▼                   ▼                 ▼
└───────────────────────────────┘ ┌────────────┐ ┌─────────────┐ ┌─────────────┐ ┌────────────────┐
                                  │AccessKey   │ │ListAccess   │ │DeleteAccess │ │  Presentation  │
                                  │ Store      │ │KeysQuery    │ │KeysCommand  │ │  Services      │
                                  └────────────┘ └──────┬──────┘ └──────┬──────┘ │├─Navigation    │
                                                        │               │        │└─ConfirmDialog │
                                                        │ inject        │ inject └────────────────┘
                                                        ▼               ▼
                                       ┌───────────────────────────────────────┐
                                       │        AccessKeyRepository            │
                                       │        (Infrastructure)               │
                                       │   Entity → Aggregate Conversion        │
                                       └───────────────────┬───────────────────┘
                                                           │ inject
                                                           ▼
                                       ┌───────────────────────────────────────┐
                                       │         AccessKeyService              │
                                       │         (Infrastructure - API)        │
                                       └───────────────────┬───────────────────┘
                                                           │ inject
                                                           ▼
                                       ┌───────────────────────────────────────┐
                                       │            HTTP_CLIENT                │
                                       │         (Infrastructure)              │
                                       └───────────────────────────────────────┘
```

---

## 📐 Naming Conventions

### General Rules

- **Variable/Method**: `snake_case` (Consistent with Rust Server)
- **Class**: `PascalCase`
- **File**: `kebab-case`

### File Naming

| Type        | Format                            | Example                            |
| ----------- | ------------------------------- | ------------------------------- |
| Component   | `{feature}-{page}.component.ts` | `access-key-list.component.ts`  |
| Facade      | `{feature}-{page}.facade.ts`    | `access-key-list.facade.ts`     |
| Store       | `{feature}.store.ts`            | `access-key.store.ts`           |
| State       | `{feature}.state.ts`            | `access-key.state.ts`           |
| Query       | `{action}-{feature}.query.ts`   | `list-access-keys.query.ts`     |
| Command     | `{action}-{feature}.command.ts` | `delete-access-keys.command.ts` |
| Repository  | `{feature}.repository.ts`       | `access-key.repository.ts`      |
| API Service | `{feature}.service.ts`          | `access-key.service.ts`         |
| Aggregate   | `{feature}.aggregate.ts`        | `access-key.aggregate.ts`       |
| Entity      | `{feature}.entity.ts`           | `access-key.entity.ts`          |
| Contract    | `{feature}.contract.ts`         | `access-key.contract.ts`        |

### Class Naming

| Type           | Format                         | Example                         |
| -------------- | ---------------------------- | ---------------------------- |
| Component      | `{Feature}{Page}Component`   | `AccessKeyListComponent`     |
| Facade         | `{Feature}{Page}Facade`      | `AccessKeyListFacade`        |
| Store          | `{Feature}Store`             | `AccessKeyStore`             |
| State          | `{Feature}State`             | `AccessKeyState`             |
| Query          | `{Action}{Feature}Query`     | `ListAccessKeysQuery`        |
| Command        | `{Action}{Feature}Command`   | `DeleteAccessKeysCommand`    |
| Repository     | `{Feature}Repository`        | `AccessKeyRepository`        |
| API Service    | `{Feature}Service`           | `AccessKeyService`           |
| Aggregate      | `{Feature}Aggregate`         | `AccessKeyAggregate`         |
| List Aggregate | `{Feature}ListItemAggregate` | `AccessKeyListItemAggregate` |

---

## ✅ Checklist

When adding a new Feature, please confirm the following items:

### Shared (Frontend + BFF Shared)

- [ ] `shared/entity/{feature}.entity.ts` - API Response Entity Definition
- [ ] `shared/contracts/{feature}.contract.ts` - DTO Contract + Validation

### Domain Layer

- [ ] `domain/aggregates/{feature}.aggregate.ts` - Domain Aggregate (Includes ListItemAggregate)

### Infrastructure Layer

- [ ] `infrastructure/api/{feature}.service.ts` - API Service
- [ ] `infrastructure/repositories/{feature}.repository.ts` - Repository (Includes Entity → Aggregate Conversion)

### Application Layer

- [ ] `application/states/{feature}.state.ts` - State Container (Signal)
- [ ] `application/stores/{feature}.store.ts` - Feature Store
- [ ] `application/queries/{feature}/` - Query Use Cases (Returns Aggregate)
- [ ] `application/commands/{feature}/` - Command Use Cases

### Presentation Layer

- [ ] `presentation/features/{feature}/` - Page Directory
- [ ] `presentation/features/{feature}/list/` - List Page + Facade
- [ ] `presentation/features/{feature}/detail/` - Detail Page + Facade
- [ ] `presentation/features/{feature}/components/` - Feature-specific Shared Components

---

## 🔄 Data Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              User Interaction                                │
└─────────────────────────────────────┬───────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  Component                                                                   │
│  ┌─────────────────┐                                                        │
│  │ facade.load()   │ ─────────────────────────────────────────────────────┐ │
│  └─────────────────┘                                                      │ │
└───────────────────────────────────────────────────────────────────────────│─┘
                                                                            │
                                      ▼                                     │
┌─────────────────────────────────────────────────────────────────────────────┐
│  Facade                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐│
│  │ store.set_loading(true);                                                ││
│  │ const result = await query.execute(params);  // Query returns Aggregate ││
│  │ store.set_items(result.rows, page);          // Store updates State     ││
│  │ store.set_loading(false);                                               ││
│  └─────────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────┬───────────────────────────────────────┘
                                      │
                      ┌───────────────┴───────────────┐
                      ▼                               ▼
┌───────────────────────────────┐     ┌───────────────────────────────────────┐
│  Query                        │     │  Store                                 │
│  ┌─────────────────────────┐  │     │  ┌─────────────────────────────────┐  │
│  │ return repository       │  │     │  │ state.items.set(items);         │  │
│  │   .find_all(params);    │  │     │  │ state.page.set(page);           │  │
│  └─────────────────────────┘  │     │  └─────────────────────────────────┘  │
└───────────────┬───────────────┘     └───────────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────────────────────────────────────────────────┐
│  Repository                                                                    │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │ const response = await api.list_keys(...);                              │  │
│  │ return {                                                                │  │
│  │   rows: response.rows.map(e => AccessKeyListItemAggregate.from(e)),     │  │
│  │   total: response.total,                                                │  │
│  │ };                                                                      │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────────────────────┘
```

---

## 🚫 Anti-Patterns

### ❌ DO NOT

```typescript
// ❌ Component calls API directly
@Component({...})
export class BadComponent {
  constructor(private http: HttpClient) {}
  load() {
    this.http.get('/api/keys').subscribe(...);
  }
}

// ❌ Query directly manipulates Store internal state
@Injectable()
export class BadQuery {
  async execute() {
    this.store.set_loading(true);    // Query should not know Store implementation
    const result = await this.repo.find_all();
    this.store.set_items(result);    // Facade should decide how to handle result
  }
}

// ❌ Store returns Entity instead of Aggregate
@Injectable()
export class BadStore {
  readonly items = signal<AccessKeyListItem[]>([]);  // Should use Aggregate
}

// ❌ Aggregate depends on Angular services
export class BadAggregate {
  constructor(private http: HttpClient) {}  // Violates Domain Layer independence
}

// ❌ Store directly calls API Service
@Injectable()
export class BadStore {
  private api = inject(AccessKeyService);  // Should go through Repository
}
```

### ✅ DO

```typescript
// ✅ Component coordinates via Facade
@Component({...})
export class GoodComponent {
  readonly facade = inject(AccessKeyListFacade);
  load() {
    this.facade.load_data(1);
  }
}

// ✅ Query only fetches data, returns result
@Injectable()
export class GoodQuery {
  async execute(params): Promise<ListResponse<AccessKeyListItemAggregate>> {
    return await this.repository.find_all(params);
  }
}

// ✅ Facade coordinates Query + Store
@Injectable()
export class GoodFacade {
  async load_data(page: number) {
    this.store.set_loading(true);
    const result = await this.query.execute({ page });
    this.store.set_items(result.rows, page);
    this.store.set_loading(false);
  }
}

// ✅ Store uses Aggregate
@Injectable()
export class GoodStore {
  readonly items = signal<AccessKeyListItemAggregate[]>([]);
}

// ✅ Aggregate is pure TypeScript implementation
export class GoodAggregate {
  private constructor(private entity: AccessKey) {}
  static from(entity: AccessKey) {
    return new GoodAggregate(entity);
  }
}

// ✅ Repository converts Entity → Aggregate
@Injectable()
export class GoodRepository {
  async find_all(): Promise<ListResponse<AccessKeyListItemAggregate>> {
    const response = await this.api.list_keys();
    return {
      rows: response.rows.map((e) => AccessKeyListItemAggregate.from(e)),
      total: response.total,
    };
  }
}
```
