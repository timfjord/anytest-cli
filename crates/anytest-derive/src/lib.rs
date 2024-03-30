use proc_macro::TokenStream;
use quote::{format_ident, quote};
use regex::{Captures, Regex};
use syn::{parse_macro_input, DeriveInput};

fn to_snake_case(value: String) -> String {
    let re1 = Regex::new(r"([A-Z]+)([A-Z][a-z])").unwrap();
    let re2 = Regex::new(r"([a-z]|\d)([A-Z])").unwrap();

    let replaced1 = re1.replace_all(&value, |caps: &Captures| {
        format!("{}_{}", &caps[1], &caps[2])
    });

    let replaced2 = re2.replace_all(&replaced1, |caps: &Captures| {
        format!("{}_{}", &caps[1], &caps[2])
    });

    replaced2.to_lowercase()
}

#[proc_macro_derive(Language)]
pub fn derive_language(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let language = input.ident;
    let language_name = format_ident!("{}", to_snake_case(language.to_string()));

    let expanded = quote! {
        impl Language for #language {
            fn name(&self) -> &str {
                stringify!(#language_name)
            }

            fn env(&self) -> &crate::registry::EnvHashMap {
                &self.env
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(TestFrameworkMeta)]
pub fn derive_test_framework_meta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let test_framework = input.ident;
    let test_framework_name = format_ident!("{}", to_snake_case(test_framework.to_string()));

    let expanded = quote! {
        impl TestFrameworkMeta for #test_framework {
            fn language(&self) -> Box<&dyn crate::registry::Language> {
                Box::new(&self.language)
            }

            fn name(&self) -> &str {
                stringify!(#test_framework_name)
            }

            fn pattern(&self) -> Result<regex::Regex, regex::Error> {
                regex::Regex::new(&self.pattern)
            }

            fn default_program(&self) -> &str {
                &self.program
            }

            fn args(&self) -> &crate::registry::ArgsVec {
                &self.args
            }

            fn env(&self) -> &crate::registry::EnvHashMap {
                &self.env
            }
        }
    };

    TokenStream::from(expanded)
}
