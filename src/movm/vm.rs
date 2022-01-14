use crate::inst::{div, dump, dup, jmp, minus, mp, plus, plusf, push, Inst, InstError, InstType};
use crate::stack::Stack;
use crate::word::Word;

pub struct VM {
    stack: Stack,
    halt: bool,
    current_ip: usize,
    total_ip: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: Stack::new(),
            halt: false,
            current_ip: 0,
            total_ip: 0,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halt
    }

    // pub fn get_inst_pointer(&self) -> usize {
    //     self.current_ip
    // }

    pub fn set_inst_pointer(&mut self, ip: usize) -> bool {
        if ip > self.total_ip {
            false
        } else {
            self.current_ip = ip;
            true
        }
    }

    pub fn get_program_length(&self) -> usize {
        self.total_ip
    }

    fn handle_error(&mut self, err: InstError, typ: &InstType, op: Word) {
        self.halt = true;
        println!("\x1b[31m{}\x1b[0m: {}", err.kind, err);
        self.stack.dump();
        println!("\x1b[93mInstruction with error:\x1b[0m");
        println!("{} |    {} {}", self.current_ip, typ, op.get_as_u64());
    }

    fn exec_inst(&mut self, inst: &Inst) {
        if self.halt {
            panic!("VM is halted");
        }

        let res = match inst.typ {
            InstType::PUSH => push(&mut self.stack, inst.op),
            InstType::PLUS => plus(&mut self.stack),
            InstType::MINUS => minus(&mut self.stack),
            InstType::MP => mp(&mut self.stack),
            InstType::DIV => div(&mut self.stack),
            InstType::DUMP => dump(&mut self.stack),
            InstType::DUP => dup(&mut self.stack, inst.op),
            InstType::PLUSF => plusf(&mut self.stack),
            InstType::MINUSF => plusf(&mut self.stack),
            InstType::MPF => plusf(&mut self.stack),
            InstType::DIVF => plusf(&mut self.stack),
            InstType::JMP => {
                let res = jmp(self, inst.op);
                self.current_ip -= 1;

                res
            }
        };

        if let Err(err) = res {
            self.handle_error(err, &inst.typ, inst.op);
        }

        self.current_ip += 1;
    }

    pub fn execute_program(&mut self, program_vec: Vec<Inst>) {
        self.total_ip = program_vec.len();

        while self.current_ip != self.total_ip && !self.is_halted() {
            self.exec_inst(program_vec.get(self.current_ip).unwrap());
        }

        self.total_ip = 0;
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}
