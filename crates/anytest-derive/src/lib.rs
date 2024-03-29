use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LanguageMeta)]
pub fn derive_language_meta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl LanguageMeta for #name {
            fn name(&self) -> &str {
                &self.name
            }

            fn env(&self) -> &EnvHashMap {
                &self.env
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(TestFrameworkMeta)]
pub fn derive_test_framework_meta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl TestFrameworkMeta for #name {
            fn language(&self) -> Box<&dyn Language> {
                Box::new(&self.language)
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn pattern(&self) -> Result<Regex, Error> {
                Regex::new(&self.pattern)
            }

            fn env(&self) -> &EnvHashMap {
                &self.env
            }
        }
    };

    TokenStream::from(expanded)
}
