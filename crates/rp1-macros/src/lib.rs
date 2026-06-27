use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    let user_ident = format_ident!("__rp1_{}", input.sig.ident);
    input.sig.ident = user_ident.clone();

    quote! {
        #input

        #[unsafe(no_mangle)]
        pub extern "Rust" fn rp1_entry() -> ! {
            let peripherals = unsafe { rp1_hal::Peripherals::steal() };
            #user_ident(peripherals)
        }
    }
    .into()
}
