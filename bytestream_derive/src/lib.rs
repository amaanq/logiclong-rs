extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data::Struct, DataEnum, DataStruct, DataUnion, DeriveInput, Field,
    Fields::Named, FieldsNamed, FieldsUnnamed, Type,
};

mod message;

/// Generate `rakrs_io::CanIo` implementation for structs and enums that have all fields implement `CanIo`
///
/// For structs, fields are written one by one in order.
///
/// For enums, the structure starts with a discriminant with the type specified in the `#[repr]` of
/// the enum, followed by the fields of the enum one by one. If the enum repr should be little
/// endian, the `#[little_endian]` attribute must be applied on the `enum` item.
#[proc_macro_derive(Message, attributes(little_endian))]
pub fn derive_message(item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as DeriveInput);
    match message::imp(parsed) {
        Ok(item) => item.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let description = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents = named.iter().map(|f| &f.ident);
                format!(
                    "a struct with these named fields: {}",
                    quote! {#(#idents), *}
                )
            }

            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let num_fields = unnamed.iter().count();
                format!("a struct with {} unnamed fields", num_fields)
            }

            syn::Fields::Unit => "a unit struct".to_string(),
        },

        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs = variants.iter().map(|v| &v.ident);
            format!("an enum with these variants: {}", quote! {#(#vs), *})
        }

        syn::Data::Union(DataUnion {
            fields: FieldsNamed { named, .. },
            ..
        }) => {
            let idents = named.iter().map(|f| &f.ident);
            format!(
                "a union with these named fields: {}",
                quote! {#(#idents), *}
            )
        }
    };

    let output = quote! {
        impl #ident {
            pub fn describe() -> &'static str {
                #description
            }
        }
    };

    output.into()
}
