use std::env::args;
use std::path::PathBuf;

pub struct Cli {
    path: PathBuf,
}

impl Cli {
    pub fn new() -> Cli {
        let path = args().nth(1).expect("no path given");
        let pathbuf = PathBuf::from(path);

        if !pathbuf.exists() {
            panic!("file does not exist");
        }

        Cli { path: pathbuf }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}
