use std::env::args;
use std::path::PathBuf;

pub struct Cli {
    input_path: PathBuf,
    output_path: PathBuf,
}

impl Cli {
    pub fn new() -> Cli {
        if args().len() != 3 {
            panic!("required 3 arguments");
        }

        let input_path = args().nth(1).expect("no input path given");
        let input_pathbuf = PathBuf::from(input_path);

        let output_path = args().nth(2).expect("no output path given");

        if !input_pathbuf.exists() {
            panic!("input file does not exist");
        }

        Cli {
            input_path: input_pathbuf,
            output_path: PathBuf::from(output_path),
        }
    }

    pub fn get_input_path(&self) -> &PathBuf {
        &self.input_path
    }

    pub fn get_output_path(&self) -> &PathBuf {
        &self.output_path
    }
}
