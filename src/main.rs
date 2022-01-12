use crate::vm::VM;

mod stack;
mod inst;
mod vm;
mod cli;
mod loader;

fn main() {
    // let cli = Cli::new();

    let mut vm = VM::new();

    // let program_vec = vec![
    //     Inst { typ: InstType::PUSH, op: 1 },   // 0
    //     Inst { typ: InstType::PUSH, op: 1 },   // 1
    //     Inst { typ: InstType::DUP,  op: 1 },   // 2
    //     Inst { typ: InstType::DUP,  op: 1 },   // 3
    //     Inst { typ: InstType::PLUS, op: 0 },   // 4
    //     Inst { typ: InstType::JMP,  op: 2 }    // 5
    // ];

    // loader::dump_program("examples/fibonacci.mobc", &program_vec);
    let loaded_program = loader::load_program("examples/fibonacci.mobc");

    vm.execute_program(loaded_program);
}
