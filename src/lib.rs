extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ImplItem, ItemImpl, Visibility};

#[proc_macro_attribute]
pub fn wasm_wrap(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = &input.self_ty;
    let unsafety = &input.unsafety;
    let generics = &input.generics;
    let trait_ = &input.trait_;
    let mut new_items = Vec::new();

    for item in &input.items {
        match item {
            ImplItem::Fn(m) => {
                let vis = &m.vis;
                let sig = &m.sig;
                let ident = &sig.ident;
                let inputs = &sig.inputs;
                let output = &sig.output;
                let block = &m.block;
                let asyncness = sig.asyncness.is_some();

                // Only wrap pub methods
                if matches!(vis, Visibility::Public(_)) {
                    let wrapped = if asyncness {
                        quote! {
                            #[wasm_bindgen]
                            #vis async fn #ident(#inputs) -> Result<JsValue, JsValue> {
                                let result: Result<_, Box<dyn std::error::Error>> = (async move { #block }).await;
                                match result {
                                    Ok(val) => serde_wasm_bindgen::to_value(&val)
                                        .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("serde error: {}", e))),
                                    Err(e) => Err(wasm_bindgen::JsValue::from_str(&format!("error: {}", e))),
                                }
                            }
                        }
                    } else {
                        quote! {
                            #[wasm_bindgen]
                            #vis fn #ident(#inputs) -> Result<JsValue, JsValue> {
                                let result: Result<_, Box<dyn std::error::Error>> = (move { #block });
                                match result {
                                    Ok(val) => serde_wasm_bindgen::to_value(&val)
                                        .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("serde error: {}", e))),
                                    Err(e) => Err(wasm_bindgen::JsValue::from_str(&format!("error: {}", e))),
                                }
                            }
                        }
                    };
                    new_items.push(wrapped);
                } else {
                    new_items.push(quote! { #m });
                }
            }
            _ => new_items.push(item.to_token_stream()),
        }
    }

    let maybe_trait = if let Some((bang, path, for_token)) = trait_ {
        quote! { #bang #path #for_token }
    } else {
        quote! {}
    };

    let result = quote! {
        #[wasm_bindgen]
        #[cfg(feature = "wasm")]
        #unsafety impl #generics #maybe_trait #self_ty {
            #(#new_items)*
        }

        #[cfg(not(feature = "wasm"))]
        #input
    };

    result.into()
}