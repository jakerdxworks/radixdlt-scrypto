use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

macro_rules! trace {
    ($($arg:expr),*) => {{
        #[cfg(feature = "trace")]
        println!($($arg),*);
    }};
}

pub fn handle_describe(input: TokenStream) -> TokenStream {
    trace!("handle_describe() starts");

    let DeriveInput { ident, data, .. } = parse2(input).expect("Unable to parse input");
    let ident_str = ident.to_string();
    trace!("Describing: {}", ident);

    let output = match data {
        Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let names = named.iter().map(|f| {
                    f.ident
                        .clone()
                        .expect("All fields must be named")
                        .to_string()
                });
                let types = named.iter().map(|f| &f.ty);

                quote! {
                    impl sbor::Describe for #ident {
                        fn describe() -> sbor::Type {
                            extern crate alloc;
                            use alloc::collections::BTreeMap;
                            use alloc::string::ToString;
                            use sbor::{self, Describe};

                            let mut fields = BTreeMap::new();
                            #(fields.insert(#names.to_string(), <#types>::describe());)*

                            sbor::Type::Struct {
                                name: #ident_str.to_string(),
                                fields: sbor::Fields::Named { fields },
                            }
                        }
                    }
                }
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let types = unnamed.iter().map(|f| &f.ty);

                quote! {
                    impl sbor::Describe for #ident {
                        fn describe() -> sbor::Type {
                            extern crate alloc;
                            use alloc::string::ToString;
                            use alloc::vec::Vec;
                            use sbor::{self, Describe};

                            let mut fields = Vec::new();
                            #(fields.push(<#types>::describe());)*

                            sbor::Type::Struct {
                                name: #ident_str.to_string(),
                                fields: sbor::Fields::Unnamed { fields },
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    impl sbor::Describe for #ident {
                        fn describe() -> sbor::Type {
                            sbor::Type::Struct {
                                name: #ident_str.to_string(),
                                fields: sbor::Fields::Unit,
                            }
                        }
                    }
                }
            }
        },
        Data::Enum(DataEnum { variants, .. }) => {
            let names = variants.iter().map(|v| v.ident.to_string());
            let types = variants.iter().map(|v| {
                let f = &v.fields;

                match f {
                    syn::Fields::Named(FieldsNamed { named, .. }) => {
                        let names = named.iter().map(|f| {
                            f.ident
                                .clone()
                                .expect("All fields must be named")
                                .to_string()
                        });
                        let types = named.iter().map(|f| &f.ty);
                        quote! {
                            {
                                let mut fields = BTreeMap::new();
                                #(fields.insert(#names.to_string(), <#types>::describe());)*
                                sbor::Fields::Named {
                                    fields,
                                }
                            }
                        }
                    }
                    syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let types = unnamed.iter().map(|f| &f.ty);
                        quote! {
                            {
                                let mut fields = Vec::new();
                                #(fields.push(<#types>::describe());)*
                                sbor::Fields::Unnamed {
                                    fields,
                                }
                            }
                        }
                    }
                    syn::Fields::Unit => {
                        quote! {
                            {
                                sbor::Fields::Unit
                            }
                        }
                    }
                }
            });

            quote! {
                impl sbor::Describe for #ident {
                    fn describe() -> sbor::Type {
                        extern crate alloc;
                        use alloc::collections::BTreeMap;
                        use alloc::string::ToString;
                        use alloc::vec::Vec;
                        use sbor::{self, Describe};

                        let mut variants = BTreeMap::new();
                        #(variants.insert(#names.to_string(), #types);)*

                        sbor::Type::Enum {
                            name: #ident_str.to_string(),
                            variants,
                        }
                    }
                }
            }
        }
        Data::Union(_) => {
            panic!("Union is not supported!")
        }
    };
    trace!("handle_derive() finishes");

    output.into()
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::str::FromStr;
    use proc_macro2::TokenStream;

    use crate::describe::handle_describe;
    use crate::utils::print_compiled_code;

    #[test]
    fn test_describe() {
        let input = TokenStream::from_str("struct Test {a: u32}").unwrap();
        let output = handle_describe(input);
        print_compiled_code("test_describe()", output);
    }
}
