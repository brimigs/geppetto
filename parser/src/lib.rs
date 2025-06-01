use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::ItemMod;

/// A very minimal Program AST: just wrap the raw `ItemMod` for now.
pub struct Program(pub ItemMod);

impl Parse for Program {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let item_mod: ItemMod = input.parse()?;
        Ok(Program(item_mod))
    }
}
