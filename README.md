# Bandana Meta Workspace

**Bandana Meta** is a Rust workspace providing metadata and schema tooling for Bevy-based games. It powers editor-like workflows (similar to Unity, Unreal, or Godot) by enabling scripts, components, and schema information to be exported, serialized, and reloaded at runtime.

This workspace consists of two crates:

* **`bandana_meta`** → Core schema types, static registries, and export support.
* **`bandana_meta_macros`** → Procedural macros (`#[script]`) for exposing scripts and functions to the editor.

---

## Workspace Layout

```
bandana_meta_exporter/
├── bandana_meta/           # Core crate
│   ├── src/lib.rs
│   └── Cargo.toml
├── bandana_meta_macros/    # Proc macro crate
│   ├── src/lib.rs
│   └── Cargo.toml
├── Cargo.toml              # Workspace manifest
├── Cargo.lock
└── .gitignore
```

Root `Cargo.toml` defines the workspace members:

```toml
[workspace]
members = [
    "bandana_meta",
    "bandana_meta_macros"
]
```

---

## Features

### 1. **Schema Representation**

Defines runtime and serialized schema types (`Schema`, `ScriptMeta`, `ParamMeta`, `ParamType`). These describe how scripts and components can be attached to entities in an editor.

### 2. **Static Registry** (behind `bandana_export` feature)

Provides a static, heap-free metadata registry using [`inventory`](https://docs.rs/inventory).

```rust
#[cfg(feature = "bandana_export")]
pub struct ScriptMetaStatic {
    pub name: &'static str,
    pub rust_symbol: &'static str,
    pub params: &'static [ParamMetaStatic],
}
```

This allows editor tooling to discover available scripts automatically.

### 3. **Export Support**

With `bandana_export` enabled, scripts can be collected and serialized into `.ron` files:

```rust
let schema = bandana_meta::collect_schema_ron_pretty();
println!("{}", schema);
```

Which generates a human-readable RON schema file for the editor:

```ron
Schema(
  scripts: [
    ScriptMeta(
      name: "Player",
      rust_symbol: "my_game::scripts::Player",
      params: [],
    ),
    ...
  ],
)
```

### 4. **Proc Macro: `#[script]`**

A simple attribute macro to expose structs or functions to the editor.

```rust
use bandana_meta_macros::script;

#[script]
pub struct Player;

#[script(name = "Teleport")]
pub fn teleport() {}
```

This registers the script into the schema export pipeline when built with `bandana_export`.

---

## Usage

### Add to Your Project

In your game’s `Cargo.toml`:

```toml
[dependencies]
bandana_meta = { path = "../bandana_meta_exporter/bandana_meta" }
bandana_meta_macros = { path = "../bandana_meta_exporter/bandana_meta_macros" }
```

### Export Schema

Build your game with the `bandana_export` feature to generate `.schema.ron`:

```bash
cargo run --bin export_schema --features bandana_export
```

This outputs a schema file (`design/.schema.ron`) that the editor can read.

### Attach Scripts in the Editor

Once the schema is available, scripts can be attached to entities in the editor. For example:

```ron
EntityDoc(
  id: "ground",
  components: [...],
  scripts: [
    AttachedScript(name: "Player", params: {}),
    AttachedScript(name: "Teleport", params: {}),
  ],
)
```

---

## 🌍 Goals

* Provide **Unity/Godot-like metadata** for Bevy without modifying the Bevy engine.
* Allow designers and tools to **attach, configure, and hot-reload scripts**.
* Keep runtime lean by using static registries + schema export only when needed.
* Enable game-specific editors to **visualize and manipulate Bevy scenes** through metadata.

---

## Extending

* Add support for **typed script parameters** (currently placeholder).
* Expand `ParamType` to cover more Bevy component types.
* Integrate with a **scene editor UI** (such as the companion `bandana_editor`).
* Publish as crates.io packages for wider Bevy ecosystem usage.

---

## License

MIT License © 2025

This project is experimental and intended for building custom Bevy tooling.
