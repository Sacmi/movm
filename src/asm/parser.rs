use movm::inst::{Inst, InstType};
use movm::word::Word;

pub fn get_inst_type_from_str(typ_str: &str) -> InstType {
    match typ_str {
        "push" => InstType::PUSH,
        "plus" => InstType::PLUS,
        "dup" => InstType::DUP,
        "jmp" => InstType::JMP,
        &_ => {
            panic!("unknown inst type")
        }
    }
}

pub fn get_inst_from_line(line: &str) -> Inst {
    assert_ne!(line.len(), 0);

    let mut split = line.trim().split_whitespace();
    let typ_str = split
        .nth(0)
        .expect("unable to get type of instruction in line");

    let typ = get_inst_type_from_str(typ_str);
    let mut op: Word = 0;

    if typ.is_required_op() {
        let op_str = split
            .last()
            .expect("unable to get operand of instruction in line");

        let op_word = op_str.to_string().parse::<Word>();

        if !op_word.is_err() {
            op = op_word.unwrap();
        } else {
            panic!("unable to parse value of operand");
        }
    }

    Inst { typ, op }
}
