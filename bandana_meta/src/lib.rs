//! bandana_meta: schema + static registry + pretty export

use serde::{Deserialize, Serialize};

/// ---------- Runtime / serialized schema (owned) ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub scripts: Vec<ScriptMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMeta {
    pub name: String,
    pub rust_symbol: String,
    pub params: Vec<ParamMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamMeta {
    pub key: String,
    pub label: String,
    pub ty: ParamType,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum ParamType {
    Bool,
    I64,
    F64,
    String,
    Vec3,
    ColorRgba,
}

/// ---------- Static metadata for inventory (no heap) ----------
#[cfg(feature = "bandana_export")]
#[derive(Debug, Clone, Copy)]
pub struct ParamMetaStatic {
    pub key: &'static str,
    pub label: &'static str,
    pub ty: ParamType,
    pub default: Option<&'static str>,
}

#[cfg(feature = "bandana_export")]
#[derive(Debug, Clone, Copy)]
pub struct ScriptMetaStatic {
    pub name: &'static str,
    pub rust_symbol: &'static str,
    pub params: &'static [ParamMetaStatic],
}

#[cfg(feature = "bandana_export")]
pub struct ScriptInventory(pub &'static ScriptMetaStatic);

/// ---------- Export support (feature-gated) ----------
#[cfg(feature = "bandana_export")]
mod export_support {
    use super::*;
    use ron::ser::PrettyConfig;

    inventory::collect!(ScriptInventory);

    pub fn collect_schema_ron_pretty() -> String {
        let scripts: Vec<ScriptMeta> = inventory::iter::<ScriptInventory>()
            .map(|entry| {
                let s = entry.0;
                let params: Vec<ParamMeta> = s
                    .params
                    .iter()
                    .map(|p| ParamMeta {
                        key: p.key.to_string(),
                        label: p.label.to_string(),
                        ty: p.ty,
                        default: p.default.map(|d| d.to_string()),
                    })
                    .collect();

                ScriptMeta {
                    name: s.name.to_string(),
                    rust_symbol: s.rust_symbol.to_string(),
                    params,
                }
            })
            .collect();

        let schema = Schema { scripts };
        ron::ser::to_string_pretty(
            &schema,
            PrettyConfig::new().struct_names(true).indentor("  "),
        )
        .expect("serialize schema")
    }
}

#[cfg(feature = "bandana_export")]
pub use export_support::collect_schema_ron_pretty;
// NOTE: don't re-export ParamMetaStatic/ScriptMetaStatic/ScriptInventory here;
// they are already defined at the crate root (behind the feature).
