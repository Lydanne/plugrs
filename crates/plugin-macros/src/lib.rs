use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn export_plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        #[no_mangle]
        pub fn create_plugin() -> Box<dyn plugin_interface::Plugin> {
            Box::new(#struct_name::new())
        }
    };

    TokenStream::from(expanded)
}
