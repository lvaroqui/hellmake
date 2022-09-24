use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn wrap_libc_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);

    let name = f.sig.ident;
    let inputs = &f.sig.inputs;
    let input_types = f.sig.inputs.iter().map(|arg| match arg {
        syn::FnArg::Receiver(_) => panic!(),
        syn::FnArg::Typed(t) => t.ty.clone(),
    });
    let input_pat = f.sig.inputs.iter().map(|arg| match arg {
        syn::FnArg::Receiver(_) => panic!(),
        syn::FnArg::Typed(t) => t.pat.clone(),
    });
    let ret = f.sig.output;
    let body = f.block;

    let tokens = quote! {
        #[no_mangle]
        /// # Safety
        ///
        /// Wraps the provided C function
        pub unsafe extern "C" fn #name(#inputs) #ret {
            lazy_static! {
                static ref FUNC: unsafe extern "C" fn(#(#input_types),*) #ret = {
                    let c_str = std::ffi::CString::new(stringify!(#name)).unwrap();
                    let func = unsafe { libc::dlsym(RTLD_NEXT, c_str.as_ptr()) };
                    assert!(!func.is_null());
                    unsafe { std::mem::transmute(func) }
                };
            }
            #body;
            FUNC(#(#input_pat),*)
        }
    };

    tokens.into()
}
