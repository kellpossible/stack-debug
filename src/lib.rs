use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn instrument(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Ensure no attributes are passed to `instrument` for now.
    // If attributes were needed, they would be parsed from `attr`.
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "#[stack_debug::instrument] does not take any arguments",
        )
        .to_compile_error()
        .into();
    }

    let block = input.block;
    let sig = input.sig;
    let vis = input.vis;
    let attrs = input.attrs;

    #[cfg(feature = "tracing")]
    let expanded = quote! {
        #(#attrs)*
        #[inline(never)]
        #[tracing::instrument(skip_all)]
        #vis #sig {
            let rbp: usize;
            let rsp: usize;

            unsafe {
                // These instructions are for x86_64.
                // Other architectures like AArch64 may have different
                // conventions or registers (e.g., `fp`).
                #[cfg(target_arch = "x86_64")]
                std::arch::asm!(
                    "mov {}, rbp", // Get the base pointer
                    "mov {}, rsp", // Get the stack pointer
                    out(reg) rbp,
                    out(reg) rsp,
                );
                // Add cfgs for other architectures if needed.
            }

            // rbp should be greater than rsp. If it's not, frame pointers
            // were likely omitted, and the result is meaningless.
            #[cfg(debug_assertions)]
            let frame_size = rbp - rsp - 1520; // subtract overhead of instrumentation
            #[cfg(not(debug_assertions))]
            let frame_size = rbp - rsp - 224; // subtract overhead of instrumentation
            tracing::info!("stack frame size: {frame_size}");

            #block
        }
    };

    #[cfg(not(feature = "tracing"))]
    let expanded = {
        let function_name = sig.ident.to_string();
        quote! {
            #(#attrs)*
            #[inline(never)]
            #vis #sig {
                let rbp: usize;
                let rsp: usize;

                unsafe {
                    // These instructions are for x86_64.
                    // Other architectures like AArch64 may have different
                    // conventions or registers (e.g., `fp`).
                    #[cfg(target_arch = "x86_64")]
                    std::arch::asm!(
                        "mov {}, rbp", // Get the base pointer
                        "mov {}, rsp", // Get the stack pointer
                        out(reg) rbp,
                        out(reg) rsp,
                    );
                    // Add cfgs for other architectures if needed.
                }

                // rbp should be greater than rsp. If it's not, frame pointers
                // were likely omitted, and the result is meaningless.
                #[cfg(debug_assertions)]
                let frame_size = rbp - rsp - 128; // subtract overhead of instrumentation
                #[cfg(not(debug_assertions))]
                let frame_size = rbp - rsp - 80; // subtract overhead of instrumentation
                println!("{}::{}(): stack frame size: {frame_size}", module_path!(), #function_name);

                #block
            }
        }
    };

    expanded.into()
}