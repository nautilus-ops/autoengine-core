use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

pub(crate) fn expand_with_node_define(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
            pub name: String,
            pub action_type: String,
            pub icon: String,
            pub output_schema: schemars::Schema,
            pub input_schema: schemars::Schema,
            #original_fields
        }
    };

    expanded.into()
}
