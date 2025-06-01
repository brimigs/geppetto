use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Attribute, ItemFn, ItemMod, PatType, Type};
// A very minimal Program AST: just wrap the raw `ItemMod` for now.
// pub struct Program(pub ItemMod);

/// FIX ME: Update the above struct to the below struct.
#[derive(Debug)]
pub struct Program {
    pub ixs: Vec<Ix>,
    pub name: Ident,
    pub docs: Option<Vec<String>>,
    pub program_mod: ItemMod,
    // pub fallbakc_fn: Option<FallbackFn>,
}
impl Parse for Program {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        // 1) Parse the raw `pub mod foo { … }` as an ItemMod
        let mut program_mod: ItemMod = input.parse()?;

        // 2) Extract module name and docs
        let name = program_mod.ident.clone();
        let docs = {
            // If you want to collect `#[doc = "..."]` into Vec<String>, do it here.
            // For now, we’ll ignore module‐level docs:
            None
        };

        // 3) Iterate through the module’s content and pull out all free functions.
        //    An inline module has `program_mod.content = Some((brace_tok, items))`.
        let mut ixs: Vec<Ix> = Vec::new();
        if let Some((_brace, items)) = &mut program_mod.content {
            for item in items.iter() {
                if let syn::Item::Fn(item_fn) = item {
                    // Found one function in the module; record it as a handler
                    let ident = item_fn.sig.ident.clone();
                    let docs = {
                        // Optionally collect `#[doc = "..."]` from item_fn.attrs into Vec<String>
                        None
                    };
                    let cfgs = item_fn.attrs.clone(); // copy any #[cfg(...)] attributes
                    ixs.push(Ix {
                        raw_method: item_fn.clone(),
                        ident,
                        docs,
                        cfgs,
                    });
                }
            }
        }

        Ok(Program {
            ixs,
            name,
            docs,
            program_mod,
        })
    }
}

impl From<&Program> for TokenStream2 {
    fn from(program: &Program) -> Self {
        // Possibly used by codegen; not strictly needed for the macro attribute itself
        program.program_mod.to_token_stream()
    }
}

#[derive(Debug)]
pub struct Ix {
    /// The raw function AST (e.g. `pub fn hello(ctx: InstructionContext) -> ProgramResult { … }`)
    pub raw_method: ItemFn,
    /// The function’s name (e.g. `hello`)
    pub ident: Ident,
    /// Any doc‐comments on that function
    pub docs: Option<Vec<String>>,
    /// Any `#[cfg]` attributes on that function
    pub cfgs: Vec<Attribute>,
    // You can add more fields (Argument parsing, return type, discriminator overrides, etc.)
}

#[derive(Debug)]
pub struct IxArg {
    pub name: Ident,
    pub docs: Option<Vec<String>>,
    pub raw_arg: PatType,
}

#[derive(Debug)]
pub struct IxReturn {
    pub ty: Type,
}

#[derive(Debug)]
pub struct FallbackFn {
    raw_method: ItemFn,
}

/// Common overrides for the `#[instruction]`, `#[account]` and `#[event]` attributes
#[derive(Debug, Default)]
pub struct Overrides {
    /// Override the default 8-byte discriminator
    pub discriminator: Option<TokenStream>,
}
