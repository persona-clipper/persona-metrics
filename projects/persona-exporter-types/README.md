# Persona Exporter Types
## RU
`persona-exporter-types` это крейт который предоставляет типы данных для `persona-exporter`.

Crate используется как единый источник структур и перечислений, которые применяются при обмене данными между компонентами проекта. Это снижает дублирование моделей и упрощает сопровождение совместимости.

Другие разработчики могут использовать эти же структуры в собственных Rust-проектах, включая веб-сайты и сервисы, которые также собирают и обрабатывают метрики. Это позволяет сохранять единый формат данных между независимыми реализациями.

### Установка

Добавить зависимость в `Cargo.toml`:

```toml
[dependencies]
persona-exporter-types = "1.0.0"
```

### Использование

Подключить crate в коде:

```rust
use persona_exporter_types::*;
```

Далее использовать публичные типы crate в местах, где требуется общий контракт данных.

### Версионирование

Используется схема `MAJOR.MINOR.PATCH` (пример: `1.0.0`).

- `MAJOR` — главное обновление; может содержать несовместимые изменения и требовать адаптации кода.
- `MINOR` — небольшое функциональное обновление; обычно обратно совместимо, но может нести интеграционные риски.
- `PATCH` — безопасный патч; исправления и мелкие улучшения без изменения публичного контракта.

Пример `1.3.2`:
- `1` — major-уровень,
- `3` — minor-уровень,
- `2` — patch-уровень.

## EN

`persona-exporter-types` is a crate that provides data types for `persona-exporter`.

The crate serves as a single source of shared structs and enums used for data exchange between project components. This reduces model duplication and simplifies compatibility maintenance.

Other developers can use the same structures in their own Rust projects, including websites and services that also collect and process metrics. This helps keep a consistent data format across independent implementations.

### Installation

Add the dependency to `Cargo.toml`:

```toml
[dependencies]
persona-exporter-types = "1.0.0"
```

### Usage

Import the crate in code:

```rust
use persona_exporter_types::*;
```

Then use the crate’s public types wherever a shared data contract is required.

### Versioning

The crate follows the `MAJOR.MINOR.PATCH` scheme (example: `1.0.0`).

- `MAJOR`: primary update; may include breaking changes and require code adaptation.
- `MINOR`: small feature update; usually backward compatible, but may carry integration risks.
- `PATCH`: safe patch; fixes and minor improvements without changing the public contract.

Example `1.3.2`:
- `1` = major level
- `3` = minor level
- `2` = patch level
