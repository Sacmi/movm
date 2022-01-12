use movm::loader::load_program;
use movm::vm::VM;
use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::new();
    let mut vm = VM::new();

    let path = cli
        .get_path()
        .as_os_str()
        .to_str()
        .expect("Runner: Unable to get path");

    let program = load_program(path);

    vm.execute_program(program);
}