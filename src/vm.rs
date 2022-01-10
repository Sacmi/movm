use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};

use crate::inst::{div, dump, dup, Inst, InstError, InstType, jmp, minus, mp, plus, push};
use crate::stack::{Stack, Word};

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
        println!("{} |    {} {}", self.current_ip, typ, op);
    }

    fn exec_inst(&mut self, inst: &Inst) {
        if self.halt {
            panic!("VM is halted");
        }

        let res: Result<(), InstError>;

        match inst.typ {
            InstType::PUSH => {
                res = push(&mut self.stack, inst.op);
            }
            InstType::PLUS => {
                res = plus(&mut self.stack);
            }
            InstType::MINUS => {
                res = minus(&mut self.stack);
            }
            InstType::MP => {
                res = mp(&mut self.stack);
            }
            InstType::DIV => {
                res = div(&mut self.stack);
            }

            InstType::DUMP => {
                res = dump(&mut self.stack);
            }
            InstType::JMP => {
                res = jmp(self, inst.op);
                self.current_ip -= 1;
            }
            InstType::DUP => {
                res = dup(&mut self.stack, inst.op);
            }
        }

        if res.is_err() {
            self.handle_error(res.unwrap_err(), &inst.typ, inst.op);
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

    pub fn dump_program(path: &str, program_vec: Vec<Inst>) -> Result<(), io::Error> {
        match bincode::serialize(&program_vec) {
            Ok(bytes) => {
                let mut file = File::create(path)?;
                file.write_all(&bytes)?;
                Ok(())
            }
            Err(error) => panic!("ERROR: unable serialize to bytes ({})", error)
        }
    }

    pub fn load_program(path: &str) -> Vec<Inst> {
        let mut f = File::open(&path).expect("no file found");
        let metadata = fs::metadata(&path).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        bincode::deserialize(&buffer[..]).expect("unable to serialize")
    }
}
