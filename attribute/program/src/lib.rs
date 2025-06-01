extern crate proc_macro;
use geppetto_parser::Program;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

/// The `#[program]` attribute defines the module containing all instruction
/// handlers defining all entries into a Solana program.
#[proc_macro_attribute]
pub fn program(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 1) Parse the user’s module into our rich AST (with ixs already populated)
    let parsed: Program = parse_macro_input!(input as Program);

    // 2) Grab the module’s identifier (e.g. "hello_world") and docs (unused here)
    let module_ident = &parsed.name;

    // 3) Collect the names of all handler fns inside the module:
    let handler_idents: Vec<&syn::Ident> = parsed.ixs.iter().map(|ix| &ix.ident).collect();

    // 4) Generate the `process_instruction` wrapper depending on how many handlers:
    let generated_entrypoint = match handler_idents.len() {
        0 => {
            // No free functions found inside the `#[program]` module
            let msg = "No handler functions found inside #[program] module!";
            quote! { compile_error!(#msg); }
        }
        1 => {
            // Exactly one handler, so forward all calls to module::that_fn(ctx)
            let handler_fn = handler_idents[0];
            quote! {
                // This macro comes from geppetto-core re‐exporting pinocchio::lazy_program_entrypoint
                geppetto_core::lazy_program_entrypoint!(process_instruction);

                // In single‐handler mode, `lazy_program_entrypoint!` will expand roughly to:
                //
                // #[no_mangle]
                // pub extern "C" fn process_instruction(input: *mut u8) -> u64 {
                //     // Build a Pinocchio InstructionContext
                //     let mut ctx = unsafe {
                //         ::pinocchio::entrypoint::lazy::InstructionContext::new_unchecked(input)
                //     };
                //     // Invoke the one function inside the module:
                //     match #module_ident::#handler_fn(ctx) {
                //         Ok(()) => ::pinocchio::SUCCESS,
                //         Err(e) => e.into(),
                //     }
                // }
            }
        }
        _ => {
            // More than one handler: you could generate a `match` on an 8‐byte discriminator here.
            // For now, emit a compile_error.
            let msg = "Multiple handler functions found; multi‐entrypoint dispatch is not yet implemented!";
            quote! { compile_error!(#msg); }
        }
    };

    // 5) Re‐emit the user’s original module tokens, exactly as written:
    let original_mod_tokens = parsed.program_mod.to_token_stream();

    // 6) Combine the generated entrypoint + user module into one final TokenStream:
    let expanded = quote! {
        #generated_entrypoint
        #original_mod_tokens
    };

    TokenStream::from(expanded)
}
