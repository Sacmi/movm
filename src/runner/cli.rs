use std::env::args;
use std::path::PathBuf;
use std::process::exit;

pub struct Cli {
    path: PathBuf,
}

impl Cli {
    pub fn new() -> Cli {
        let path = args().nth(1).expect("Runner: no path given");
        let pathbuf = PathBuf::from(path);

        if !pathbuf.exists() {
            println!("Runner: file does not exist.");
            exit(1);
        }

        Cli { path: pathbuf }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}
