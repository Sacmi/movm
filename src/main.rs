use crate::inst::{Inst, InstType};
use crate::vm::VM;

mod stack;
mod inst;
mod vm;

fn main() {
    let mut vm = VM::new();

    // let program_vec = vec![
    //     Inst { typ: InstType::PUSH, op: 1 },   // 0
    //     Inst { typ: InstType::PUSH, op: 1 },   // 1
    //     Inst { typ: InstType::DUP,  op: 1 },   // 2
    //     Inst { typ: InstType::DUP,  op: 1 },   // 3
    //     Inst { typ: InstType::PLUS, op: 0 },   // 4
    //     Inst { typ: InstType::JMP,  op: 2 }    // 5
    // ];

    let loaded_program = VM::load_program("examples/fibonacci.bin");
    // VM::dump_program("fibonacci.bin", program_vec);

    vm.execute_program(loaded_program);
}
