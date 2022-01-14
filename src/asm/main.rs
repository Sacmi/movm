use crate::cli::Cli;
use crate::parser::get_inst_from_line;
use movm::inst::Inst;
use movm::loader;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod cli;
mod parser;

fn main() {
    let cli = Cli::new();
    let input_str = cli
        .get_input_path()
        .to_str()
        .expect("unable to get str of input path");

    let file = File::open(input_str).expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut insts: Vec<Inst> = Vec::new();

    for line_res in reader.lines() {
        let line = line_res.expect("unable to get line from input file");

        let inst = get_inst_from_line(line.as_str());
        insts.push(inst);
    }

    let output_str = cli
        .get_output_path()
        .to_str()
        .expect("unable to get output path");

    loader::dump_program(output_str, &insts);
}
