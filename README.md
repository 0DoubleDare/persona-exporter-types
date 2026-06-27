# Persona Exporter: Types

Набор Rust-типов для проекта `persona-exporter`.

## Установка

Добавьте crate в `Cargo.toml`:

```toml
[dependencies]
persona-exporter-types = "0.1"
```

## Пример использования (rustdoc)

```rust
/// Пример: создание структуры с типами из crate.
///
/// ```
/// use persona_exporter_types::{Persona, TraitScore};
///
/// let persona = Persona {
///     id: "user-123".to_string(),
///     name: "Алиса".to_string(),
///     traits: vec![
///         TraitScore {
///             key: "openness".to_string(),
///             score: 0.82,
///         },
///         TraitScore {
///             key: "conscientiousness".to_string(),
///             score: 0.74,
///         },
///     ],
/// };
///
/// assert_eq!(persona.id, "user-123");
/// assert_eq!(persona.traits.len(), 2);
/// ```
fn rustdoc_example() {}
```

## Назначение

- типобезопасный обмен данными между сервисами;
- единый контракт структур `persona-exporter`;
- удобная сериализация/десериализация (например, через `serde`, если включено в crate).
