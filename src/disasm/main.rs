use movm::loader;

use crate::cli::Cli;

mod cli;

fn get_space_count(length: &usize) -> usize {
    let mut counter = 0;
    let mut number = *length;

    while number != 0 {
        counter += 1;
        number /= 10;
    }

    counter
}

fn main() {
    let cli = Cli::new();

    let path = cli
        .get_path()
        .as_os_str()
        .to_str()
        .expect("unable to get path");

    let program = loader::load_program(path);
    let spaces = get_space_count(&program.len());

    for i in 0..program.len() {
        let inst = program[i];

        print!("{:>space$} | \x1b[31m{}\x1b[0m", i, inst.typ.to_string().to_lowercase(), space = spaces);

        if inst.is_required_op() {
            print!(" \x1b[33m{}\x1b[0m", inst.op);
        }

        println!()
    }
}
