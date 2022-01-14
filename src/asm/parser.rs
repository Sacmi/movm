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
        .next()
        .expect("unable to get type of instruction in line");

    let typ = get_inst_type_from_str(typ_str);
    let mut op: Word = Word::new_u64(0);

    if typ.is_required_op() {
        let op_str = split
            .last()
            .expect("unable to get operand of instruction in line")
            .to_string();

        if op_str.contains('.') {
            let op_f64 = op_str
                .parse::<f64>()
                .expect("unable to parse this operand as float");

            op = Word::new_f64(op_f64);
        } else {
            let op_i64 = op_str.parse::<i64>();

            if let Ok(op_ok) = op_i64 {
                op = Word::new_i64(op_ok);
            } else {
                let op_u64 = op_str
                    .parse::<u64>()
                    .expect("unable to parse operand as u64");
                op = Word::new_u64(op_u64)
            }
        }
    }

    Inst { typ, op }
}
