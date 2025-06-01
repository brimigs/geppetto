use geppetto_attribute_program::program;
use pinocchio::{entrypoint::InstructionContext, log, ProgramResult};

fn process_instruction(ctx: InstructionContext) -> ProgramResult {
    hello_world::hello(ctx)
}
#[program]
pub mod hello_world {
    use super::*;
    pub fn hello(_ctx: InstructionContext) -> ProgramResult {
        log::sol_log("Hello, Geppetto!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{instruction::Instruction, pubkey};
    #[test]
    fn test_log() {
        let program_id = pubkey!("22222222222222222222222222222222222222222222");
        let mollusk = Mollusk::new(&program_id, "target/deploy/hello_world");
        mollusk.process_instruction(&Instruction::new_with_bytes(program_id, b"", vec![]), &[]);
    }
}
