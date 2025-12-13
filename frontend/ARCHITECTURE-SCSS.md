# SCSS & OOCSS Design Guidelines

本指南旨在為專案建立統一的 SCSS 開發標準，核心採用 **OOCSS (Object Oriented CSS)** 方法論，並結合 Angular Component 架構特性。

## 1. 核心哲學 (Core Philosophy)

### OOCSS 原則
1.  **結構與外觀分離 (Separate Structure and Skin)**
    - 將佈局屬性 (Structure) 與視覺裝飾屬性 (Skin) 分開定義。
    - **Structure**: `width`, `height`, `padding`, `margin`, `display`, `position`
    - **Skin**: `color`, `background`, `border`, `box-shadow`, `font-family`
    - *優點*: 相同的視覺風格可套用到不同結構的元件上，反之亦然。

2.  **容器與內容分離 (Separate Container and Content)**
    - 避免樣式依賴特定的父層容器。元件應該在任何地方都能獨立運作且樣式一致。
    - *Bad*: `.sidebar .list-item { ... }` (依賴 sidebar)
    - *Good*: `.list-item { ... }` (獨立元件)

## 2. 命名規範 (Naming Convention)

採用 **BEM (Block Element Modifier)** 命名法，以提高可讀性並減少命名衝突。

Format: `.block__element--modifier`

-   **Block**: 獨立的實體 (e.g., `.card`, `.btn`, `.menu`)
-   **Element**: Block 的一部分，無法獨立存在 (e.g., `.card__title`, `.menu__item`)
-   **Modifier**: 狀態或變體 (e.g., `.btn--primary`, `.card--featured`)

### Angular Component 特例
Encapsulated styles (預設) 有助於隔離，但在全域樣式或共用組件中務必遵守 BEM。

## 3. 最佳實踐 (Best Practices)

### 3.1 變數使用 (Variables)
-   **CSS Custom Properties (`--var`)**: 用於與主題相關、需要在 Runtime 切換的值 (Colors, Spacing, Fonts)。
-   **SCSS Variables (`$var`)**: 用於編譯期計算、斷點 (Breakpoints)、靜態路徑。

```scss
// Good
.btn {
  background-color: var(--garage-primary); // 支援主題切換
  padding: var(--space-md);
}

// Bad
.btn {
  background-color: #007bff; // Magic Color
}
```

### 3.2 巢狀與耦合 (Nesting & Coupling)
-   **限制巢狀深度**: 最多不超過 3 層。過深的巢狀會增加權重 (Specificity) 並降低效能。
-   **避免 `::ng-deep`**: 嚴禁使用。若需修改子組件樣式，應透過 Input 或 Global Theme Class。

### 3.3 數值管理 (Values)
-   **No Magic Numbers**: 所有間距、尺寸應使用變數系統。
-   **Spacing System**:
    -   `--space-xs`: 4px
    -   `--space-sm`: 8px
    -   `--space-md`: 16px
    -   `--space-lg`: 24px
    -   `--space-xl`: 32px

## 4. 範例對照 (Examples)

### Bad Example (Coupled & Skinned)
```scss
// access-key-create.component.scss
.header-left {
  display: flex;
  
  // 結構與外觀混雜，且依賴標籤
  button { 
    background: red;     // Skin
    margin-left: 10px;   // Structure
  }
}
```

### Good Example (OOCSS + BEM)
```scss
// Structure (Layout)
.header-actions {
  display: flex;
  gap: var(--space-sm); // 使用變數
}

// Component (Skin & Structure defined in component)
.btn {
  // Base styles
  padding: var(--space-sm) var(--space-md);
  border-radius: 4px;
}

// Skin (Modifier)
.btn--danger {
  background-color: var(--garage-danger);
  color: white;
}
```

## 5. CSS 屬性順序 (Property Ordering)
建議依照以下順序排列屬性，增加可讀性：
1.  **Positioning**: `position`, `top`, `z-index`
2.  **Display & Box Model**: `display`, `width`, `height`, `margin`, `padding`
3.  **Typography**: `font`, `line-height`, `color`
4.  **Visual**: `background`, `border`, `opacity`
5.  **Misc**: `transition`, `cursor`
