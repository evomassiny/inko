//! Functions for testing instruction handlers.
use binding::Binding;
use block::Block;
use compiled_code::CompiledCode;
use config::Config;
use module::Module;
use process::RcProcess;
use vm::instruction::{Instruction, InstructionType};
use vm::machine::Machine;
use vm::state::State;

/// Sets up a VM with a single process.
pub fn setup() -> (Machine, Block, RcProcess) {
    let state = State::new(Config::new());
    let machine = Machine::default(state);
    let code = CompiledCode::new("a".to_string(), "a".to_string(), 1, Vec::new());

    let (block, process) = {
        let mut registry = write_lock!(machine.module_registry);

        // To ensure the module sticks around long enough we'll manually store in
        // in the module registry.
        registry.add_module("test", Module::new(code));

        let module = match registry.get_or_set(&"test") {
            Ok(module) => module,
            Err(_) => panic!("The test module does not exist"),
        };

        let scope = module.global_scope_ref();

        let block = Block::new(module.code(), Binding::new(), scope);
        let process = machine.allocate_process(0, &block);

        (block, process.unwrap())
    };

    (machine, block, process)
}

/// Creates a new instruction.
pub fn new_instruction(ins_type: InstructionType, args: Vec<u16>) -> Instruction {
    Instruction::new(ins_type, args, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm::instruction::InstructionType;

    #[test]
    fn test_new_instruction() {
        let ins = new_instruction(InstructionType::SetInteger, vec![1, 2]);

        assert_eq!(ins.instruction_type, InstructionType::SetInteger);
        assert_eq!(ins.arguments, vec![1, 2]);
        assert_eq!(ins.line, 1);
    }
}
