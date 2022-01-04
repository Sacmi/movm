use std::env::args;
use std::path::PathBuf;

pub enum CliMode {
    Compile,
    Run,
}

pub struct Cli {
    mode: CliMode,
    path: PathBuf
}

impl Cli {
    pub fn new() -> Cli {
        let mode = args().nth(1).expect("Cli: no mode given");
        let path = args().nth(2).expect("Cli: no path given");

        let cli_mode: CliMode = match mode.as_str() {
            "compile" => CliMode::Compile,
            "run" => CliMode::Run,
            _ => panic!("Cli: unknown mode given")
        };

        Cli {
            mode: cli_mode,
            path: PathBuf::from(path)
        }
    }

    pub fn get_mode(&self) -> &CliMode {
        &self.mode
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}