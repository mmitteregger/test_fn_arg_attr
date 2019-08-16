#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::Span;
use syn::{Error, FnArg};

use quote::quote;

#[proc_macro_attribute]
pub fn test_attr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function_result = syn::parse::<syn::ItemFn>(input)
        .map_err(|error| error.span().unstable().error(error.to_string()))
        .map_err(|d| d.help(String::from("#[test_attr] can only be used on functions")));

    let mut function = match function_result {
        Ok(function) => function,
        Err(diag) => {
            diag.emit();
            return TokenStream::new();
        }
    };

    let mut function_sig_args_iter = function.sig.inputs.iter_mut();
    let arg_with_foo_attr_present = match function_sig_args_iter.next() {
        Some(first_fn_arg) => match first_fn_arg {
            FnArg::Receiver(_receiver) => {
                emit_error(
                    "first #[test_attr] function argument should not be any variant of `self`",
                );
                return TokenStream::new();
            }
            FnArg::Typed(pat_type) => {
                let mut arg_with_foo_attr_present = false;

                pat_type.attrs.retain(|fn_arg_attr| {
                    if fn_arg_attr.path.is_ident("foo") {
                        arg_with_foo_attr_present = true;
                        false
                    } else {
                        true
                    }
                });

                arg_with_foo_attr_present
            }
        },
        None => {
            emit_error("#[test_attr] function needs at least one argument");
            return TokenStream::new();
        }
    };
    drop(function_sig_args_iter);

    if arg_with_foo_attr_present {
        let token_stream = quote! {
            // this does not make much sense,
            // it's only an simple example for code generation
            // depending on function argument attributes
            #function
        };
        token_stream.into()
    } else {
        emit_error("first #[test_attr] function argument should annotated with `#[foo]`");
        TokenStream::new()
    }
}

fn emit_error(error_message: &str) {
    let error = Error::new(Span::call_site(), error_message);
    let diag = error.span().unstable().error(error.to_string());
    diag.emit();
}
