#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_parse_ident() {
        let ident: syn::Ident = parse_quote!(MyStruct);
        assert_eq!(ident.to_string(), "MyStruct");
    }

    #[test]
    fn test_parse_type() {
        let ty: syn::Type = parse_quote!(Vec<String>);
        assert!(matches!(ty, syn::Type::Path(_)));
    }

    #[test]
    fn test_parse_item_struct() {
        let item: syn::ItemStruct = parse_quote!(
            struct MyStruct {
                field: String,
            }
        );
        assert_eq!(item.ident.to_string(), "MyStruct");
        assert_eq!(item.fields.len(), 1);
    }

    #[test]
    fn test_quote_macro() {
        let tokens = quote! {
            struct MyStruct {
                field: String,
            }
        };
        let parsed: syn::ItemStruct = syn::parse2(tokens).unwrap();
        assert_eq!(parsed.ident.to_string(), "MyStruct");
    }
}
