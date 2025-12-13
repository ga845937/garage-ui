# Rust gRPC Server Architecture Guidelines

This document describes the layered architecture design principles for the Garage UI Backend, based on Clean Architecture and CQRS (Command Query Responsibility Segregation) concepts.

> **Note**: This project uses a Monorepo structure where the Rust Server and Frontend coexist. All naming conventions are unified using `snake_case`.

---

## 📁 Directory Structure

```
.
├── proto/                         # Protocol Buffers Definitions (Project Root)
│   ├── access_key.proto
│   ├── bucket.proto
│   └── utility.proto              # Common types (Pagination, NullableString, etc.)
│
└── src/
    ├── main.rs                    # Application Entry Point
    ├── lib.rs                     # Module Exports
    │
    ├── domain/                    # Domain Layer - Core Business Logic
    │   ├── aggregates/            # Aggregate Roots (Business Logic + Validation)
    │   │   └── access_key.rs      # AccessKeyAggregate
    │   ├── entities/              # Domain Entities / Read Models
    │   │   ├── access_key.rs      # AccessKey, AccessKeyListItem
    │   │   └── garage/            # Raw Garage API Response Structures (Shared with Entity)
    │   ├── repositories/          # Repository Abstractions (Traits)
    │   │   └── access_key_repository.rs
    │   ├── value_objects/         # Value Objects
    │   ├── events/                # Domain Events (TODO: To be implemented)
    │   └── errors.rs              # Domain Error Definitions
    │
    ├── application/               # Application Layer - Use Case Coordination
    │   ├── commands/              # Write Operations (CQRS Command)
    │   │   └── access_key/
    │   │       ├── mod.rs
    │   │       ├── create_key.rs  # CreateKeyCommand
    │   │       ├── update_key.rs  # UpdateKeyCommand
    │   │       ├── delete_key.rs  # DeleteKeyCommand
    │   │       └── handlers/      # Command Handlers
    │   └── queries/               # Read Operations (CQRS Query)
    │       └── access_key/
    │           ├── mod.rs
    │           ├── list_keys.rs   # ListKeysQuery
    │           ├── read_key.rs    # ReadKeyQuery
    │           └── handlers/      # Query Handlers
    │
    ├── infrastructure/            # Infrastructure Layer - Technical Implementation
    │   ├── grpc/                  # gRPC Services
    │   │   ├── server.rs          # gRPC Server Startup
    │   │   ├── services/          # gRPC Service Implementations
    │   │   ├── composition/       # DI Composition (Builder Pattern)
    │   │   ├── generated/         # Protobuf Generated Code
    │   │   ├── conversions.rs     # Type Conversions + Error Handling
    │   │   ├── middleware.rs      # gRPC Middleware
    │   │   └── logging.rs         # Logging Utilities
    │   ├── garage/                # Garage Admin API Client
    │   │   ├── client.rs          # HTTP Client
    │   │   ├── endpoints.rs       # API Path Definitions
    │   │   ├── api/               # API Request/Response Structures
    │   │   └── repositories/      # Repository Implementations
    │   ├── config.rs              # Configuration Management
    │   └── logging.rs             # Logging Initialization
    │
    └── shared/                    # Cross-Layer Shared Utilities
        ├── context.rs             # Request Context (trace_id)
        ├── datetime.rs            # Date/Time Parsing
        ├── pagination.rs          # Pagination Utilities
        ├── update_field.rs        # Update Field Tristate Semantics
        └── trace_id.rs            # Trace ID Generation
```

> **Note**: `domain/entities/` and `domain/entities/garage/` are shared because the Domain Entity and Garage API response structures are currently almost identical. To avoid an unnecessary translation layer, structures are shared directly.
>
> **Note**: `shared/` only contains utility functions (pagination, datetime, etc.) and does not contain business-related DTOs.

---

## 🏗️ Layered Architecture

### Dependency Rule (Clean Architecture)

```
                    ┌─────────────────────────┐
                    │   Infrastructure Layer  │
                    │   (gRPC Service, API)   │
                    └───────────┬─────────────┘
                                │ Depends on ↓
                    ┌───────────▼─────────────┐
                    │    Application Layer    │
                    │   (Command, Query,      │
                    │    Handler)             │
                    └───────────┬─────────────┘
                                │ Depends on ↓
┌───────────────────────────────▼───────────────────────────────┐
│                        Domain Layer                            │
│           (Aggregate, Entity, Repository Trait)                │
│                         【Core Layer】                          │
100: └───────────────────────────────▲───────────────────────────────┘
                                │ Depends on ↑ (Dependency Inversion)
                    ┌───────────┴─────────────┐
                    │   Infrastructure Layer  │
                    │ (Repository Impl, HTTP) │
                    └─────────────────────────┘
```

**Key Principles:**

- **Domain Layer is Core**: Does not depend on any external frameworks or infrastructure.
- **Dependency Inversion**: Domain defines Repository Traits, Infrastructure implements them.
- **CQRS Separation**: Commands and Queries have independent Repository interfaces.
- **Aggregate Manages Business Rules**: All validation and state changes occur within the Aggregate.

---

## 📦 Layer Responsibilities

### 1. Infrastructure Layer

#### gRPC Service

- **Responsibilities**: Receives gRPC requests, converts them to Command/Query, calls Handler.
- **Location**: `infrastructure/grpc/services/`
- **Dependencies**: Command/Query Handlers

```rust
pub struct AccessKeyGrpcService {
    create_key_handler: Arc<CreateKeyHandler>,
    update_key_handler: Arc<UpdateKeyHandler>,
    delete_key_handler: Arc<DeleteKeyHandler>,
    list_keys_handler: Arc<ListKeysHandler>,
    read_key_handler: Arc<ReadKeyHandler>,
}

#[tonic::async_trait]
impl AccessKeyService for AccessKeyGrpcService {
    async fn create_key(
        &self,
        request: Request<CreateKeyRequest>,
    ) -> Result<Response<KeyResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("AccessKeyService", "CreateKey", &req);
        let trace_id = get_trace_id();

        // 1. Create Command
        let command = CreateKeyCommand::new(
            req.name,
            req.expiration,
            req.allow_create_bucket,
        );

        // 2. Call Handler (Returns Aggregate)
        let aggregate = self.create_key_handler
            .handle(command)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        // 3. Aggregate → DTO, Convert to gRPC Response
        let response = KeyResponse {
            trace_id: trace_id.clone(),
            data: Some(Key::from_aggregate(&aggregate)),  // Mapper Conversion
        };

        log.ok(&response);
        Ok(Response::new(response))
    }
}
```

**gRPC Service Responsibility Classification:**

| Responsibility Category | Description | Example |
| ------------------- | ----------------------------- | --------------------------------- |
| **Request Parsing** | Extract data from gRPC Request | `request.into_inner()` |
| **Command Creation** | Convert to Application Layer Command | `CreateKeyCommand::new(...)` |
| **Handler Call** | Delegate to corresponding Handler | `handler.handle(command).await` |
| **Error Conversion** | DomainError → gRPC Status | `domain_error_to_status(e)` |
| **Aggregate → DTO** | Convert Aggregate to gRPC Message | `Key::from_aggregate(&aggregate)` |
| **Logging** | Request/Response Logging | `grpc_log!(...)` |

> **Guideline**: Handler returns Aggregate, gRPC Service is responsible for using Mapper to convert to DTO (gRPC Message).
> This keeps the Application Layer domain-oriented, while the Infrastructure Layer handles protocol conversion.

#### Service Composition (DI Builder)

- **Responsibilities**: Composes all dependencies for the Service (Dependency Injection).
- **Location**: `infrastructure/grpc/composition/`
- **Design Concept**: Separates DI logic from server.rs, allowing the server to focus on startup.

```rust
pub struct AccessKeyServiceBuilder {
    client: GarageClient,
}

impl AccessKeyServiceBuilder {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }

    pub fn build(self) -> AccessKeyGrpcService {
        // Create Repositories (Separate Command and Query)
        let command_repository = Arc::new(
            GarageAccessKeyCommandRepository::new(self.client.clone())
        );
        let query_repository = Arc::new(
            GarageAccessKeyQueryRepository::new(self.client.clone())
        );

        // Create Command Handlers
        let create_key_handler = Arc::new(CreateKeyHandler::new(command_repository.clone()));
        let update_key_handler = Arc::new(UpdateKeyHandler::new(command_repository.clone()));
        let delete_key_handler = Arc::new(DeleteKeyHandler::new(command_repository));

        // Create Query Handlers
        let list_keys_handler = Arc::new(ListKeysHandler::new(query_repository.clone()));
        let read_key_handler = Arc::new(ReadKeyHandler::new(query_repository));

        // Compose gRPC Service
        AccessKeyGrpcService::new(
            create_key_handler,
            update_key_handler,
            delete_key_handler,
            list_keys_handler,
            read_key_handler,
        )
    }
}
```

#### Repository Implementation

- **Responsibilities**: Implements Repository Traits defined in the Domain Layer.
- **Location**: `infrastructure/garage/repositories/`
- **Dependencies**: GarageClient (HTTP Client)

> ⚠️ **Guideline**: All Repositories **must return Aggregates**. The gRPC Service is responsible for converting to DTOs.
> This matches the Frontend architecture, ensuring the Application Layer always operates on domain objects.

```rust
// ============ Command Repository ============
// For write operations, returns Aggregate

pub struct GarageAccessKeyCommandRepository {
    client: GarageClient,
}

#[async_trait]
impl AccessKeyCommandRepository for GarageAccessKeyCommandRepository {
    async fn get(&self, id: &str) -> Result<AccessKeyAggregate, DomainError> {
        let response: KeyInfoResponse = self.client.get(&path).await?;
        Ok(map_response_to_aggregate(response))  // API Response → Aggregate
    }

    async fn create(&self, aggregate: &AccessKeyAggregate) -> Result<AccessKeyAggregate, DomainError> {
        let request = CreateKeyRequest::from_aggregate(aggregate);
        let response: KeyInfoResponse = self.client.post(path, &request).await?;
        Ok(map_response_to_aggregate(response))
    }

    async fn save(&self, aggregate: &AccessKeyAggregate) -> Result<AccessKeyAggregate, DomainError> {
        let request = UpdateKeyRequest::from_aggregate(aggregate);
        let response: KeyInfoResponse = self.client.post(&path, &request).await?;
        Ok(map_response_to_aggregate(response))
    }

    async fn delete(&self, aggregate: &AccessKeyAggregate) -> Result<(), DomainError> {
        self.client.post_empty(&path).await
    }
}

// ============ Query Repository ============
// For read operations, also returns Aggregate (Consistent with Frontend)

pub struct GarageAccessKeyQueryRepository {
    client: GarageClient,
}

#[async_trait]
impl AccessKeyQueryRepository for GarageAccessKeyQueryRepository {
    async fn list(&self) -> Result<Vec<AccessKeyAggregate>, DomainError> {
        let response: Vec<KeyInfoResponse> = self.client.get(path).await?;
        Ok(response.into_iter().map(map_response_to_aggregate).collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<AccessKeyAggregate, DomainError> {
        let response: KeyInfoResponse = self.client.get(&path).await?;
        Ok(map_response_to_aggregate(response))
    }
}
```

> **Note**: Filtering and pagination are handled in memory within the Query Handler because the Garage Admin API does not support server-side filtering.
> This is a known performance limitation for large datasets.

#### Garage Client (HTTP Client)

- **Responsibilities**: Encapsulates HTTP calls to the Garage Admin API.
- **Location**: `infrastructure/garage/client.rs`
- **Features**: Unified error handling, logging, request tracing.

```rust
pub struct GarageClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl GarageClient {
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, DomainError> {
        // HTTP GET + Error Handling + Logging
    }

    pub async fn post<T: DeserializeOwned, R: Serialize>(
        &self,
        path: &str,
        body: &R
    ) -> Result<T, DomainError> {
        // HTTP POST + Error Handling + Logging
    }
}
```

---

### 2. Application Layer

#### Command (Write Operations)

- **Responsibilities**: Encapsulates input data for write operations.
- **Location**: `application/commands/{feature}/`
- **Design Concept**: Commands are immutable data structures carrying all information needed for an operation. **Validation is handled by the Aggregate**.

```rust
/// Command to create a new access key
#[derive(Debug, Clone)]
pub struct CreateKeyCommand {
    name: String,
    expiration: Option<DateTime<Utc>>,
    allow_create_bucket: bool,
}

impl CreateKeyCommand {
    pub fn new(
        name: String,
        expiration: Option<String>,
        allow_create_bucket: Option<bool>,
    ) -> Self {
        Self {
            name,
            expiration: expiration.and_then(|s| parse_datetime(&s)),
            allow_create_bucket: allow_create_bucket.unwrap_or(false),
        }
    }

    // Getters
    pub fn name(&self) -> &str { &self.name }
    pub fn expiration(&self) -> Option<DateTime<Utc>> { self.expiration }
    pub fn allow_create_bucket(&self) -> bool { self.allow_create_bucket }

    // Note: Command does NOT validate. Validation is unified in the Aggregate.
}
```

**Command Responsibilities:**

| Responsibility Category | Description | Necessity |
| ------------ | -------------------------- | ------ |
| **Data Encapsulation** | Encapsulate all inputs required for the operation | Required |
| **Type Conversion** | String → DateTime, etc. | Required |
| **Default Values** | Handle default values for Optional parameters | Required |

> **Guideline**: Command does not perform validation. All business validation is unified in the Aggregate to avoid scattered rules.

#### Command Handler

- **Responsibilities**: Coordinates the flow of a Command, creating/loading Aggregates, and persisting them. Does NOT contain business logic.
- **Location**: `application/commands/{feature}/handlers/`
- **Dependencies**: Command Repository (via Trait)

```rust
pub struct CreateKeyHandler {
    repository: Arc<dyn AccessKeyCommandRepository>,
}

impl CreateKeyHandler {
    pub fn new(repository: Arc<dyn AccessKeyCommandRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: CreateKeyCommand) -> Result<AccessKeyAggregate, DomainError> {
        // 1. Create via Aggregate (Business validation is in Aggregate::new)
        let new_aggregate = AccessKeyAggregate::new(
            command.name().to_string(),
            command.expiration(),
            command.allow_create_bucket(),
        )?;

        // 2. Persist
        let aggregate = self.repository.create(&new_aggregate).await?;

        // 3. Return Aggregate (Converted to DTO by gRPC Service)
        Ok(aggregate)
    }
}
```

**Command Handler Flow:**

```
1. Create/Load Aggregate      → Aggregate::new() or repository.get()
2. Execute Business Operation → aggregate.rename() / aggregate.apply_update()
3. Persist via Repository     → repository.save(&aggregate)
4. Return Aggregate           → gRPC Service handles DTO conversion
```

#### Query (Read Operations)

- **Responsibilities**: Defines query specifications (filtering, pagination, sorting criteria).
- **Location**: `application/queries/{feature}/`
- **Design Concept**: Query is the encapsulation of query conditions.

```rust
/// Query to list all access keys
#[derive(Debug, Clone, Default)]
pub struct ListKeysQuery {
    // Pagination
    pub page: i32,
    pub page_size: i32,

    // Filters
    pub name: Option<String>,
    pub created_start: Option<DateTime<Utc>>,
    pub created_end: Option<DateTime<Utc>>,
    pub expiration_start: Option<DateTime<Utc>>,
    pub expiration_end: Option<DateTime<Utc>>,
}

impl ListKeysQuery {
    pub fn new(page: i32, page_size: i32) -> Self {
        Self { page, page_size, ..Default::default() }
    }

    /// Create Query from gRPC Request (Builder Pattern)
    pub fn from_grpc_request(
        page: i32,
        page_size: i32,
        name: Option<String>,
        // ...
    ) -> Self {
        Self::new(page, page_size)
            .with_name(name)
            .with_created_range(created_start, created_end)
    }

    // Builder methods
    pub fn with_name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    /// Check if filters exist
    pub fn has_filter(&self) -> bool {
        self.name.is_some() || self.created_start.is_some() || ...
    }

    /// Check if item matches filters
    pub fn matches(&self, item: &AccessKeyAggregate) -> bool {
        // Filter logic
    }
}
```

#### Query Handler

- **Responsibilities**: Executes the query, applies filtering and pagination, returns Aggregate.
- **Location**: `application/queries/{feature}/handlers/`
- **Dependencies**: Query Repository (via Trait)

```rust
pub struct ListKeysHandler {
    repository: Arc<dyn AccessKeyQueryRepository>,
}

impl ListKeysHandler {
    pub fn new(repository: Arc<dyn AccessKeyQueryRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: ListKeysQuery
    ) -> Result<(Vec<AccessKeyAggregate>, usize), DomainError> {
        // 1. Get Aggregates from Repository
        let rows = self.repository.list().await?;

        // 2. Apply filters (In-memory filtering as Garage API lacks server-side filtering)
        let filtered: Vec<_> = if query.has_filter() {
            rows.into_iter()
                .filter(|item| query.matches(item))
                .collect()
        } else {
            rows
        };

        // 3. Calculate total count (Before pagination)
        let total = filtered.len();

        // 4. Apply pagination
        let paginated = paginate(&filtered, query.page, query.page_size);

        // 5. Return Aggregates (Converted to DTO by gRPC Service)
        Ok((paginated, total))
    }
}
```

---

### 3. Domain Layer

#### Aggregate (Aggregate Root)

- **Responsibilities**: Encapsulates business rules, validation logic, state changes.
- **Location**: `domain/aggregates/`
- **Should NOT have**: External dependencies (HTTP, Database, Framework).

> ⚠️ **Guideline**: All business validation must be within the Aggregate to ensure rule consistency.

```rust
/// Access Key Aggregate Root
#[derive(Debug, Clone)]
pub struct AccessKeyAggregate {
    id: String,
    name: String,
    buckets: Vec<BucketVO>,
    created: DateTime<Utc>,
    expiration: Option<DateTime<Utc>>,
    can_create_bucket: bool,
    secret_access_key: String,
}

impl AccessKeyAggregate {
    // ============ Factory Methods ============

    /// Create new Access Key (For Create operation)
    pub fn new(
        name: String,
        expiration: Option<DateTime<Utc>>,
        can_create_bucket: bool,
    ) -> Result<Self, DomainError> {
        // Business Validation
        Self::validate_name(&name)?;

        Ok(Self {
            id: String::new(),  // Filled by Repository
            name,
            buckets: vec![],
            created: Utc::now(),
            expiration,
            can_create_bucket,
            secret_access_key: String::new(),
        })
    }

    /// Reconstitute Aggregate from persistence (No validation)
    /// Only used in Infrastructure Layer Repository
    pub fn reconstitute(
        id: String,
        name: String,
        // ... other fields
    ) -> Self {
        Self { id, name, /* ... */ }
    }

    // ============ Business Operations ============

    /// Rename Access Key
    pub fn rename(&mut self, new_name: String) -> Result<(), DomainError> {
        Self::validate_name(&new_name)?;
        self.name = new_name;
        Ok(())
    }

    /// Apply update (Using UpdateField tristate semantics)
    pub fn apply_update(
        &mut self,
        name: UpdateField<String>,
        expiration: UpdateField<DateTime<Utc>>,
        can_create_bucket: UpdateField<bool>,
    ) -> Result<(), DomainError> {
        if let UpdateField::Set(new_name) = name {
            self.rename(new_name)?;
        }

        match expiration {
            UpdateField::NoChange => {}
            UpdateField::Clear => self.expiration = None,
            UpdateField::Set(exp) => self.expiration = Some(exp),
        }

        // ...
        Ok(())
    }

    /// Check if expired (Business logic)
    pub fn is_expired(&self) -> bool {
        self.expiration.map(|exp| exp < Utc::now()).unwrap_or(false)
    }

    // ============ Validation (Public for Command usage) ============

    pub fn validate_name(name: &str) -> Result<(), DomainError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(DomainError::ValidationError(
                "Access key name cannot be empty".to_string()
            ));
        }
        if trimmed.len() > 128 {
            return Err(DomainError::ValidationError(
                "Access key name must be at most 128 characters".to_string()
            ));
        }
        Ok(())
    }

    pub fn validate_expiration_future(exp: Option<DateTime<Utc>>) -> Result<(), DomainError> {
        if let Some(exp) = exp {
            if exp < Utc::now() {
                return Err(DomainError::ValidationError(
                    "Expiration must be in the future".to_string()
                ));
            }
        }
        Ok(())
    }

    // ============ Getters ============
    pub fn id(&self) -> &str { &self.id }
    pub fn name(&self) -> &str { &self.name }
    // ...
}
```

**Aggregate Responsibility Classification:**

| Responsibility Category | Description | Example |
| ---------------------- | -------------------- | ------------------------------------------------- |
| **Factory Method** | Create new Aggregate | `new()`, `reconstitute()` |
| **Business Operation** | Business logic (State change) | `rename()`, `apply_update()` |
| **Validation** | Business validation rules | `validate_name()`, `validate_expiration_future()` |
| **Query Method** | Computed properties | `is_expired()`, `bucket_count()` |

#### Value Object (VO)

- **Responsibilities**: Immutable value encapsulation, usually used inside Aggregates.
- **Location**: `domain/aggregates/` or `domain/value_objects/`

```rust
#[derive(Debug, Clone)]
pub struct BucketPermissionVO {
    owner: bool,
    read: bool,
    write: bool,
}

impl BucketPermissionVO {
    pub fn new(owner: bool, read: bool, write: bool) -> Self {
        Self { owner, read, write }
    }

    pub fn owner(&self) -> bool { self.owner }
    pub fn read(&self) -> bool { self.read }
    pub fn write(&self) -> bool { self.write }
}
```

#### Entity / Read Model

- **Responsibilities**: Data structure returned by query operations (DTO).
- **Location**: `domain/entities/`
- **Design Concept**: Read Model is a projection of the Aggregate used for Queries.

```rust
/// Access Key Details (Read Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessKey {
    pub id: String,
    pub name: String,
    pub secret_access_key: String,
    pub created: DateTime<Utc>,
    pub expiration: Option<DateTime<Utc>>,
    pub expired: bool,
    pub permissions: KeyPermissions,
    pub buckets: Vec<KeyBucket>,
}

impl AccessKey {
    /// Convert Aggregate to Read Model
    pub fn from_aggregate(aggregate: AccessKeyAggregate) -> Self {
        AccessKey {
            id: aggregate.id().to_string(),
            name: aggregate.name().to_string(),
            // ...
        }
    }
}
```

#### Repository Trait

- **Responsibilities**: Defines abstract interface for data access (Dependency Inversion).
- **Location**: `domain/repositories/`
- **Design Concept**: All Repositories **unified to return Aggregates**.

> ⚠️ **Guideline**: Command and Query Repositories both return Aggregates, consistent with Frontend architecture.
> gRPC Service is responsible for converting Aggregate to DTO.

```rust
/// Command Repository - For write operations
#[async_trait]
pub trait AccessKeyCommandRepository: Send + Sync {
    /// Get Aggregate for business logic processing
    async fn get(&self, id: &str) -> Result<AccessKeyAggregate, DomainError>;

    /// Create Access Key
    async fn create(&self, aggregate: &AccessKeyAggregate) -> Result<AccessKeyAggregate, DomainError>;

    /// Save Access Key (Accepts Aggregate)
    async fn save(&self, aggregate: &AccessKeyAggregate) -> Result<AccessKeyAggregate, DomainError>;

    /// Delete Access Key
    async fn delete(&self, aggregate: &AccessKeyAggregate) -> Result<(), DomainError>;
}

/// Query Repository - For read operations (Also returns Aggregate)
#[async_trait]
pub trait AccessKeyQueryRepository: Send + Sync {
    /// List all Access Keys (Returns Aggregates)
    async fn list(&self) -> Result<Vec<AccessKeyAggregate>, DomainError>;

    /// Get Access Key details
    async fn find_by_id(&self, id: &str) -> Result<AccessKeyAggregate, DomainError>;
}
```

#### Domain Error

- **Responsibilities**: Define Domain Layer error types.
- **Location**: `domain/errors.rs`

```rust
#[derive(Error, Debug)]
pub enum DomainError {
    // ============ Validation Errors ============
    #[error("Validation error: {0}")]
    ValidationError(String),

    // ============ Not Found Errors ============
    #[error("Access key not found: {0}")]
    AccessKeyNotFound(String),

    #[error("Bucket not found: {0}")]
    BucketNotFound(String),

    // ============ Conflict Errors ============
    #[error("Access key already exists: {0}")]
    AccessKeyAlreadyExists(String),

    // ============ Infrastructure Errors ============
    #[error("Garage API error: {0}")]
    GarageApiError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
```

---

### 4. Shared Layer

- **Responsibilities**: Utility functions shared across layers.
- **Location**: `shared/`
- **Should NOT have**: Business logic, layer dependencies.

```rust
// shared/update_field.rs
/// Update Field Tristate Semantics
pub enum UpdateField<T> {
    NoChange,   // No update
    Clear,      // Clear (Set to None/Default)
    Set(T),     // Set to new value
}

// shared/pagination.rs
pub fn paginate<T: Clone>(items: &[T], page: usize, page_size: usize) -> Vec<T> {
    let start = (page - 1) * page_size;
    items.iter().skip(start).take(page_size).cloned().collect()
}

// shared/datetime.rs
pub fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&Utc))
}
```

---

## 🔄 DI Diagram

Example using `AccessKey`:

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              GrpcServer (main.rs)                                │
└───────────────────────────────────────┬─────────────────────────────────────────┘
                                        │ builds
                                        ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                       AccessKeyServiceBuilder (composition)                      │
│  ┌────────────────────────────────────────────────────────────────────────────┐ │
│  │ 1. Create Repositories (Command + Query)                                   │ │
│  │ 2. Create Handlers (inject Repository)                                     │ │
│  │ 3. Create gRPC Service (inject Handlers)                                   │ │
│  └────────────────────────────────────────────────────────────────────────────┘ │
└───────────────────────────────────────┬─────────────────────────────────────────┘
                                        │ returns
                                        ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         AccessKeyGrpcService                                     │
│                          (Infrastructure)                                        │
└──────────────┬───────────────────────────────────────────┬──────────────────────┘
               │                                           │
    ┌──────────┴──────────┐                     ┌──────────┴──────────┐
    ▼                     ▼                     ▼                     ▼
┌─────────────┐    ┌─────────────┐       ┌─────────────┐       ┌─────────────┐
│ CreateKey   │    │ UpdateKey   │       │ ListKeys    │       │ ReadKey     │
│ Handler     │    │ Handler     │       │ Handler     │       │ Handler     │
│(Application)│    │(Application)│       │(Application)│       │(Application)│
└──────┬──────┘    └──────┬──────┘       └──────┬──────┘       └──────┬──────┘
       │                  │                     │                     │
       └────────┬─────────┘                     └─────────┬───────────┘
                │                                         │
                ▼                                         ▼
┌───────────────────────────────┐       ┌───────────────────────────────────────┐
│ AccessKeyCommandRepository    │       │    AccessKeyQueryRepository           │
│ (Domain Trait)                │       │    (Domain Trait)                     │
└───────────────┬───────────────┘       └─────────────────┬─────────────────────┘
                │                                         │
                │        ┌────────────────────────────────┘
                │        │
                ▼        ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                   GarageAccessKeyCommandRepository                               │
│                   GarageAccessKeyQueryRepository                                 │
│                   (Infrastructure - implements Trait)                            │
└───────────────────────────────────────┬─────────────────────────────────────────┘
                                        │ uses
                                        ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              GarageClient                                        │
│                          (Infrastructure - HTTP)                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 📐 Naming Conventions

### General Rules

- **Variable/Method/Field**: `snake_case`
- **Type/Struct/Enum**: `PascalCase`
- **Constant**: `SCREAMING_SNAKE_CASE`
- **Module/File**: `snake_case`

### File Naming

| Type             | Format                          | Example                    |
| ---------------- | ------------------------------- | -------------------------- |
| Command          | `{action}_{feature}.rs`         | `create_key.rs`            |
| Command Handler  | `{action}_{feature}_handler.rs` | `create_key_handler.rs`    |
| Query            | `{action}_{feature}.rs`         | `list_keys.rs`             |
| Query Handler    | `{action}_{feature}_handler.rs` | `list_keys_handler.rs`     |
| Aggregate        | `{feature}.rs`                  | `access_key.rs`            |
| Entity           | `{feature}.rs`                  | `access_key.rs`            |
| Repository Trait | `{feature}_repository.rs`       | `access_key_repository.rs` |
| Repository Impl  | `{feature}_repository.rs`       | `access_key_repository.rs` |
| gRPC Service     | `{feature}_service.rs`          | `access_key_service.rs`    |
| Service Builder  | `{feature}.rs`                  | `access_key.rs`            |

### Type Naming

| Type            | Format                            | Example                            |
| --------------- | --------------------------------- | ---------------------------------- |
| Command         | `{Action}{Feature}Command`        | `CreateKeyCommand`                 |
| Command Handler | `{Action}{Feature}Handler`        | `CreateKeyHandler`                 |
| Query           | `{Action}{Feature}Query`          | `ListKeysQuery`                    |
| Query Handler   | `{Action}{Feature}Handler`        | `ListKeysHandler`                  |
| Aggregate       | `{Feature}Aggregate`              | `AccessKeyAggregate`               |
| Read Model      | `{Feature}` / `{Feature}ListItem` | `AccessKey`, `AccessKeyListItem`   |
| Value Object    | `{Name}VO`                        | `BucketPermissionVO`               |
| Command Repo    | `{Feature}CommandRepository`      | `AccessKeyCommandRepository`       |
| Query Repo      | `{Feature}QueryRepository`        | `AccessKeyQueryRepository`         |
| Repo Impl       | `Garage{Feature}{Type}Repository` | `GarageAccessKeyCommandRepository` |
| gRPC Service    | `{Feature}GrpcService`            | `AccessKeyGrpcService`             |
| Service Builder | `{Feature}ServiceBuilder`         | `AccessKeyServiceBuilder`          |

---

## ✅ Checklist

When adding a new Feature, please confirm the following items:

### Proto

- [ ] `proto/{feature}.proto` - gRPC Service Definition

### Domain Layer

- [ ] `domain/aggregates/{feature}.rs` - Aggregate Root (Including Value Objects)
- [ ] `domain/entities/{feature}.rs` - Read Model (DTO)
- [ ] `domain/repositories/{feature}_repository.rs` - Repository Trait (Command + Query)
- [ ] `domain/errors.rs` - Add relevant Error types

### Application Layer

- [ ] `application/commands/{feature}/` - Command directory
- [ ] `application/commands/{feature}/{action}_{feature}.rs` - Command definition
- [ ] `application/commands/{feature}/handlers/{action}_{feature}_handler.rs` - Handler
- [ ] `application/queries/{feature}/` - Query directory
- [ ] `application/queries/{feature}/{action}_{feature}.rs` - Query definition
- [ ] `application/queries/{feature}/handlers/{action}_{feature}_handler.rs` - Handler

### Infrastructure Layer

- [ ] `infrastructure/garage/repositories/{feature}_repository.rs` - Repository Implementation
- [ ] `infrastructure/grpc/services/{feature}_service.rs` - gRPC Service
- [ ] `infrastructure/grpc/composition/{feature}.rs` - Service Builder

---

## 🔄 Data Flow

### Command Flow (Write Operation)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                           gRPC Request                                        │
└─────────────────────────────────┬────────────────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  gRPC Service                                                                 │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │ 1. let command = CreateKeyCommand::new(req.name, req.expiration, ...); │  │
│  │ 2. let key = handler.handle(command).await?;                           │  │
│  │ 3. Ok(Response::new(KeyResponse { data: convert(key) }))               │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────┬────────────────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  Command Handler                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │ 1. command.validate()?;                                                │  │
│  │ 2. let aggregate = Aggregate::new(...)?;  // Business Validation       │  │
│  │ 3. let saved = repository.create(&aggregate).await?;                   │  │
│  │ 4. Ok(ReadModel::from_aggregate(saved))                                │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────┬────────────────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  Command Repository (Infrastructure)                                         │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │ 1. let request = CreateKeyRequest::from(aggregate);                    │  │
│  │ 2. let response = client.post(path, &request).await?;                  │  │
│  │ 3. Ok(map_response_to_aggregate(response))  // Return Aggregate        │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Query Flow (Read Operation)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                           gRPC Request                                        │
└─────────────────────────────────┬────────────────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  gRPC Service                                                                 │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │ 1. let query = ListKeysQuery::from_grpc_request(page, page_size, ...); │  │
│  │ 2. let (aggregates, total) = handler.handle(query).await?;             │  │
│  │ 3. let data = aggregates.iter().map(KeyListItem::from).collect();      │  │
│  │ 4. Ok(Response::new(ListKeysResponse { data, total }))                 │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────┬────────────────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  Query Handler                                                                │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │ 1. let rows = repository.list().await?;  // Return Aggregates          │  │
│  │ 2. let filtered = rows.filter(|r| query.matches(r));                   │  │
│  │ 3. let paginated = paginate(&filtered, query.page, query.page_size);   │  │
│  │ 4. Ok((paginated, total))  // Return Aggregates                        │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────┬────────────────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  Query Repository (Infrastructure)                                           │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │ 1. let response = client.get(path).await?;                             │  │
│  │ 2. Ok(response.map(map_to_aggregate))  // Return Aggregates            │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 🛑 Unified Error Handling

### DomainError Definition

All business logic errors are defined in `domain/errors.rs`:

```rust
#[derive(Error, Debug)]
pub enum DomainError {
    // Validation
    #[error("Validation error: {0}")]
    ValidationError(String),

    // Not Found (404)
    #[error("Bucket not found: {0}")]
    BucketNotFound(String),
    #[error("Access key not found: {0}")]
    AccessKeyNotFound(String),
    // ... other NotFound errors

    // Already Exists (409)
    #[error("Bucket already exists: {0}")]
    BucketAlreadyExists(String),
    // ... other AlreadyExists errors

    // Precondition Failed (412)
    #[error("Layout version mismatch: expected {expected}, got {actual}")]
    LayoutVersionMismatch { expected: i64, actual: i64 },

    // Infrastructure (500)
    #[error("Garage API error: {0}")]
    GarageApiError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}
```

### Unified Error Conversion

Conversion from `DomainError` to gRPC `Status` is unified in `infrastructure/grpc/conversions.rs`:

```rust
/// Convert Domain Error to gRPC Status
pub fn domain_error_to_status(err: DomainError) -> Status {
    match err {
        // Validation → INVALID_ARGUMENT (400)
        DomainError::ValidationError(msg) => Status::invalid_argument(msg),
        DomainError::InvalidBucketName(msg) => Status::invalid_argument(format!("Invalid bucket name: {}", msg)),

        // Not Found → NOT_FOUND (404)
        DomainError::BucketNotFound(msg) => Status::not_found(msg),
        DomainError::AccessKeyNotFound(msg) => Status::not_found(msg),
        // ...

        // Already Exists → ALREADY_EXISTS (409)
        DomainError::BucketAlreadyExists(msg) => Status::already_exists(msg),
        // ...

        // Precondition → FAILED_PRECONDITION (412)
        DomainError::LayoutVersionMismatch { expected, actual } => {
            Status::failed_precondition(format!("Layout version mismatch: expected {}, got {}", expected, actual))
        }

        // Infrastructure → INTERNAL (500)
        DomainError::GarageApiError(msg) => Status::internal(format!("Garage API error: {}", msg)),
        DomainError::InternalError(msg) => Status::internal(format!("Internal error: {}", msg)),
    }
}

/// Extension trait for Result<T, DomainError>
pub trait DomainErrorExt<T> {
    fn into_grpc_result(self) -> Result<T, Status>;
}

impl<T> DomainErrorExt<T> for Result<T, DomainError> {
    fn into_grpc_result(self) -> Result<T, Status> {
        self.map_err(domain_error_to_status)
    }
}
```

### gRPC Service Usage

```rust
use crate::infrastructure::grpc::conversions::domain_error_to_status;

#[tonic::async_trait]
impl AccessKeyService for AccessKeyGrpcService {
    async fn create_key(&self, request: Request<CreateKeyRequest>) -> Result<Response<KeyResponse>, Status> {
        let req = request.into_inner();

        // Method 1: Use map_err for direct conversion
        let aggregate = self
            .create_key_handler
            .handle(command)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        // Method 2: Use DomainErrorExt trait (Optional)
        // let aggregate = self.create_key_handler.handle(command).await.into_grpc_result()?;

        Ok(Response::new(KeyResponse {
            data: Some(Key::from_aggregate(&aggregate))
        }))
    }
}
```

### Error Handling Best Practices

1. **Domain Layer**: Define only `DomainError`, unaware of gRPC Status.
2. **Application Layer**: Handler returns `Result<T, DomainError>`.
3. **Infrastructure Layer**: gRPC Service unifies conversion using `domain_error_to_status()`.
4. **Unified Mapping**: All `DomainError` variants have corresponding Status codes in `conversions.rs`.
5. **Logging**: Log error details before conversion (e.g., `log.err()` in the example).

---

## 🚫 Anti-Patterns

### ❌ DO NOT

```rust
// ❌ gRPC Service calls Repository directly
impl AccessKeyService for BadService {
    async fn create_key(&self, request: Request<CreateKeyRequest>) -> ... {
        // Should go through Handler and Aggregate
        let key = self.repository.create(request.into()).await?;
    }
}

// ❌ Handler contains business logic
impl CreateKeyHandler {
    pub async fn handle(&self, command: CreateKeyCommand) -> ... {
        // Validation should be in Aggregate, not in Handler
        if command.name().len() > 128 {
            return Err(DomainError::ValidationError("Name too long".into()));
        }
    }
}

// ❌ Command performs validation (Should be Aggregate's responsibility)
impl CreateKeyCommand {
    pub fn validate(&self) -> Result<(), DomainError> {
        // Validation should be in Aggregate::new(), not in Command
        AccessKeyAggregate::validate_name(&self.name)?;
        Ok(())
    }
}

// ❌ Aggregate depends on Infrastructure
impl AccessKeyAggregate {
    pub async fn save(&self, client: &GarageClient) -> ... {
        // Aggregate should not know how to persist
        client.post(...).await
    }
}

// ❌ Using reconstitute outside Repository
impl SomeHandler {
    pub fn handle(&self) {
        // reconstitute can ONLY be used in Repository!
        let aggregate = AccessKeyAggregate::reconstitute(...);
    }
}

// ❌ Handler returns Read Model (Should return Aggregate)
impl CreateKeyHandler {
    pub async fn handle(&self, command: CreateKeyCommand) -> Result<AccessKey, ...> {
        // Should return Aggregate, converted by gRPC Service
        Ok(AccessKey::from_aggregate(aggregate))
    }
}
```

### ✅ DO

```rust
// ✅ gRPC Service coordinates via Handler, handles Aggregate → DTO conversion
impl AccessKeyService for GoodService {
    async fn create_key(&self, request: Request<CreateKeyRequest>) -> ... {
        let command = CreateKeyCommand::new(req.name, req.expiration, ...);
        let aggregate = self.create_handler.handle(command).await?;
        Ok(Response::new(KeyResponse { data: Some(Key::from(&aggregate)) }))
    }
}

// ✅ Handler only coordinates flow, returns Aggregate
impl CreateKeyHandler {
    pub async fn handle(&self, command: CreateKeyCommand) -> Result<AccessKeyAggregate, ...> {
        // Validation executed inside Aggregate::new
        let aggregate = AccessKeyAggregate::new(command.name(), ...)?;
        let saved = self.repository.create(&aggregate).await?;
        Ok(saved)  // Return Aggregate
    }
}

// ✅ Aggregate encapsulates all business rules and validation
impl AccessKeyAggregate {
    pub fn new(name: String, ...) -> Result<Self, DomainError> {
        Self::validate_name(&name)?;  // Validation here
        Ok(Self { name, ... })
    }
}

// ✅ reconstitute used only in Repository
impl GarageAccessKeyCommandRepository {
    async fn get(&self, id: &str) -> Result<AccessKeyAggregate, ...> {
        let response = self.client.get(&path).await?;
        // Only Repository can use reconstitute
        Ok(AccessKeyAggregate::reconstitute(response.id, response.name, ...))
    }
}

// ✅ All Repositories unified to return Aggregate
#[async_trait]
impl AccessKeyQueryRepository for GoodQueryRepo {
    async fn list(&self) -> Result<Vec<AccessKeyAggregate>, ...> {
        let response = self.client.get(path).await?;
        Ok(response.into_iter().map(map_to_aggregate).collect())
    }
}
```
