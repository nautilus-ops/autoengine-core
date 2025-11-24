use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Field, Fields, ItemEnum, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn with_metadata(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析输入的 enum
    let input = parse_macro_input!(item as ItemEnum);

    let enum_ident = input.ident;
    let enum_attrs = input.attrs;

    let mut match_arms = Vec::new();

    let variants = input.variants.into_iter().map(|v| {
        let ident = v.ident;
        let attrs = v.attrs;

        // metadata 字段
        let metadata_field: Field = syn::parse_quote! {
            #[serde(flatten)]
            metadata: MetaData
        };

        // 处理不同类型的 variant
        let new_fields = match v.fields {
            // Unit variant -> 转成 struct variant，只有 metadata
            Fields::Unit => {
                match_arms.push(quote! {
                    #enum_ident::#ident { metadata, .. } => metadata
                });

                Fields::Named(syn::FieldsNamed {
                    brace_token: Default::default(),
                    named: {
                        let mut fields = Punctuated::new();
                        fields.push(metadata_field);
                        fields
                    },
                })
            }

            // Tuple variant -> 转成 struct variant，字段命名为 0,1,2...
            Fields::Unnamed(fields) => {
                let mut named = Punctuated::new();
                named.push(metadata_field);

                for (i, f) in fields.unnamed.into_iter().enumerate() {
                    let idx = syn::Index::from(i);
                    let ty = f.ty;
                    let new_field: Field = syn::parse_quote! {
                        #idx: #ty
                    };
                    named.push(new_field);
                }

                match_arms.push(quote! {
                    #enum_ident::#ident { metadata, .. } => metadata
                });

                Fields::Named(syn::FieldsNamed {
                    brace_token: Default::default(),
                    named,
                })
            }

            // Named variant -> 注入 metadata 字段
            Fields::Named(fields) => {
                let mut new_fields = Punctuated::new();
                new_fields.push(metadata_field);

                for f in fields.named {
                    new_fields.push(f);
                }

                match_arms.push(quote! {
                    #enum_ident::#ident { metadata, .. } => metadata
                });

                Fields::Named(syn::FieldsNamed {
                    brace_token: Default::default(),
                    named: new_fields,
                })
            }
        };

        quote! {
            #(#attrs)*
            #ident #new_fields
        }
    });

    let output = quote! {
        #(#enum_attrs)*
        pub enum #enum_ident {
            #(#variants),*
        }

        impl #enum_ident {
            pub fn metadata(&self) -> &MetaData {
                match self {
                    #(#match_arms),*
                }
            }
        }
    };

    output.into()
}

use syn::Path;

/// 用法: #[with_parameter(MouseClickParams)]
#[proc_macro_attribute]
pub fn with_parameter(attr: TokenStream, item: TokenStream) -> TokenStream {
    let param_ty = parse_macro_input!(attr as Path);

    let ast = parse_macro_input!(item as ItemStruct);
    let ident = &ast.ident;
    let attrs = &ast.attrs;
    let vis = &ast.vis;
    let fields = &ast.fields;

    let original_fields = match fields {
        syn::Fields::Named(named) => {
            let fields = &named.named;
            quote! { #fields }
        }
        _ => {
            return syn::Error::new_spanned(fields, "#[with_parameter] only supports named struct")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        #(#attrs)*
        #vis struct #ident {
            pub params: Option<#param_ty>,
            #original_fields
        }

        impl #ident {
            pub fn params(&self) -> Option<&#param_ty> {
                self.params.as_ref()
            }

            pub fn params_mut(&mut self) -> Option<&mut #param_ty> {
                self.params.as_mut()
            }
        }
    };

    expanded.into()
}
