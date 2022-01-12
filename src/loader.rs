use std::fs::{File, metadata};
use std::io::{Read, Write};
use std::mem::size_of;
use std::slice::from_raw_parts;

use crate::inst::Inst;

unsafe fn inst_to_bytes(p: &[Inst]) -> &[u8] {
    from_raw_parts(
        (p as *const [Inst]) as *const u8,
        p.len() * size_of::<Inst>(),
    )
}

unsafe fn bytes_to_inst(p: &[u8]) -> &[Inst] {
    from_raw_parts(
        (p as *const [u8]) as *const Inst,
        p.len() / size_of::<Inst>(),
    )
}

pub fn load_program(path: &str) -> Vec<Inst> {
    let mut file = File::open(&path).expect("Error: Unable to open file");
    let metadata = metadata(&path).expect("Error: Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Error: Buffer overflow");

    let slice = buffer.as_slice();
    let program = unsafe { bytes_to_inst(slice) };

    program.to_vec()
}

pub fn dump_program(path: &str, program_vec: &Vec<Inst>) {
    let slice = program_vec.as_slice();
    let bytes = unsafe { inst_to_bytes(slice) };

    let mut file = File::create(path).expect("Error: unable to create file.");
    file.write_all(bytes).expect("Error: Unable dump program to file.");
}