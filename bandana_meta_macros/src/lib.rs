//! bandana_meta_macros

use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, ItemFn, ItemStruct, LitStr, parse::Parser, parse_macro_input};

/// Simple attribute args: #[script] or #[script(name = "Custom")]
struct ScriptArgs {
    name: Option<LitStr>,
}

impl ScriptArgs {
    fn parse(args: proc_macro2::TokenStream) -> syn::Result<Self> {
        let mut out = ScriptArgs { name: None };

        // syn v2 key-value parser
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") {
                let lit: LitStr = meta.value()?.parse()?;
                out.name = Some(lit);
                Ok(())
            } else {
                Err(meta.error("unsupported attribute key (only `name = \"...\"` is supported)"))
            }
        });

        parser.parse2(args)?;
        Ok(out)
    }
}

/// `#[script]` – attach to a `struct` or free `fn` to expose it to the editor.
/// Optional: `#[script(name = "Friendly Name")]`
#[proc_macro_attribute]
pub fn script(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args_ts = proc_macro2::TokenStream::from(attr);
    let args = match ScriptArgs::parse(args_ts) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };

    let item_parsed = parse_macro_input!(item as Item);

    // Determine identifier and default name
    let (ident, default_name) = match &item_parsed {
        Item::Struct(ItemStruct { ident, .. }) => (ident.clone(), ident.to_string()),
        Item::Fn(ItemFn { sig, .. }) => (sig.ident.clone(), sig.ident.to_string()),
        other => {
            return syn::Error::new_spanned(
                other,
                "#[script] only supports `struct` and free `fn` items",
            )
            .to_compile_error()
            .into();
        }
    };

    let display_name = args
        .name
        .unwrap_or_else(|| LitStr::new(&default_name, ident.span()));

    // Fully-qualified symbol literal (&'static str)
    let fq_symbol =
        quote! { ::core::concat!(::core::module_path!(), "::", ::core::stringify!(#ident)) };

    // Emit original item + a feature-gated inventory submission with *static* fields.
    // No consts or named modules are created (avoids E0015 and E0428).
    let expanded = quote! {
        #item_parsed

        #[cfg(feature = "bandana_export")]
        ::inventory::submit!{
            ::bandana_meta::ScriptInventory(
                &::bandana_meta::ScriptMetaStatic{
                    name: #display_name,
                    rust_symbol: #fq_symbol,
                    params: &[], // add real params later
                }
            )
        }
    };

    expanded.into()
}
