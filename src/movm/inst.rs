use std::fmt;

use crate::stack::Stack;
use crate::vm::VM;
use crate::word::Word;

#[derive(Debug, PartialEq)]
pub enum InstErrorKind {
    NotEnoughOperands,
    DivisionByZero,
    StackOverflow,
    IllegalPointer,
    IndexOutOfRange,
    Overflow,
}

#[derive(Debug, PartialEq)]
pub struct InstError {
    pub(super) kind: InstErrorKind,
}

impl InstError {
    pub fn __description(&self) -> &str {
        match self.kind {
            InstErrorKind::NotEnoughOperands => "not enough operands at stack",
            InstErrorKind::DivisionByZero => "tried divide by zero",
            InstErrorKind::StackOverflow => "stack is full",
            InstErrorKind::IllegalPointer => "illegal pointer position",
            InstErrorKind::IndexOutOfRange => "index out of range",
            InstErrorKind::Overflow => "variable is overflowing",
        }
    }
}

impl fmt::Display for InstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum InstType {
    PUSH,
    PLUS,
    MINUS,
    MP,
    DIV,
    DUMP,
    JMP,
    DUP,
}

impl InstType {
    pub fn is_required_op(&self) -> bool {
        matches!(self, InstType::PUSH | InstType::JMP | InstType::DUP)
    }
}

impl fmt::Display for InstType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for InstErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone)]
pub struct Inst {
    pub typ: InstType,
    pub op: Word,
}

macro_rules! check_overflow {
    ($overflow:expr, $a:expr, $b:expr, $stack:expr) => {
        if $overflow.is_none() {
            $stack.push(Word::new_i64($b)).unwrap();
            $stack.push(Word::new_i64($a)).unwrap();

            return Err(InstError {
                kind: InstErrorKind::Overflow,
            });
        }
    };
}

macro_rules! check_operands {
    ($a:expr, $b:expr) => {
        if $a.get_size() < $b {
            return Err(InstError {
                kind: InstErrorKind::NotEnoughOperands,
            });
        }
    };
}

pub fn push(stack: &mut Stack, op: Word) -> Result<(), InstError> {
    let res = stack.push(op);

    if res.is_ok() {
        Ok(())
    } else {
        Err(InstError {
            kind: InstErrorKind::StackOverflow,
        })
    }
}

pub fn plus(stack: &mut Stack) -> Result<(), InstError> {
    check_operands!(stack, 2);

    let a = stack.pop().unwrap().get_as_i64();
    let b = stack.pop().unwrap().get_as_i64();

    let overflow = a.checked_add(b);
    check_overflow!(overflow, a, b, stack);

    stack.push(Word::new_i64(a + b)).unwrap();

    Ok(())
}

pub fn minus(stack: &mut Stack) -> Result<(), InstError> {
    check_operands!(stack, 2);

    let a = stack.pop().unwrap().get_as_i64();
    let b = stack.pop().unwrap().get_as_i64();

    stack.push(Word::new_i64(a - b)).unwrap();

    Ok(())
}

pub fn mp(stack: &mut Stack) -> Result<(), InstError> {
    check_operands!(stack, 2);

    let a = stack.pop().unwrap().get_as_i64();
    let b = stack.pop().unwrap().get_as_i64();

    let overflow = a.checked_mul(b);
    check_overflow!(overflow, a, b, stack);

    stack.push(Word::new_i64(a * b)).unwrap();

    Ok(())
}

pub fn div(stack: &mut Stack) -> Result<(), InstError> {
    check_operands!(stack, 2);

    let a = stack.pop().unwrap().get_as_i64();
    let b = stack.pop().unwrap().get_as_i64();

    if b == 0 {
        return Err(InstError {
            kind: InstErrorKind::DivisionByZero,
        });
    }

    let overflow = a.checked_div(b);
    check_overflow!(overflow, a, b, stack);

    stack.push(Word::new_i64(a / b)).unwrap();
    Ok(())
}

pub fn dump(stack: &mut Stack) -> Result<(), InstError> {
    stack.dump();
    Ok(())
}

pub fn jmp(vm: &mut VM, op: Word) -> Result<(), InstError> {
    if op.get_as_u64() > vm.get_program_length() as u64 {
        return Err(InstError {
            kind: InstErrorKind::IllegalPointer,
        });
    }

    vm.set_inst_pointer(op.get_as_u64() as usize);

    Ok(())
}

pub fn dup(stack: &mut Stack, op: Word) -> Result<(), InstError> {
    if op.get_as_u64() >= stack.get_size() as u64 {
        return Err(InstError {
            kind: InstErrorKind::IndexOutOfRange,
        });
    }

    let duplicate = stack
        .at(stack.get_size() - 1 - (op.get_as_u64() as usize))
        .unwrap();
    let res = stack.push(duplicate);

    if res.is_err() {
        Err(InstError {
            kind: InstErrorKind::StackOverflow,
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_test() {
        let mut stack = Stack::new();

        push(&mut stack, Word::new_i64(69)).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap().get_as_i64(), 69);
    }

    #[test]
    fn plus_test() {
        let mut stack = Stack::new();

        stack.push(Word::new_i64(15)).unwrap();
        stack.push(Word::new_i64(20)).unwrap();

        plus(&mut stack).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap().get_as_i64(), 35);
    }

    #[test]
    fn macro_test() {
        let mut stack = Stack::new();
        let err = plus(&mut stack);

        assert_eq!(
            err.unwrap_err(),
            InstError {
                kind: InstErrorKind::NotEnoughOperands
            }
        )
    }

    #[test]
    fn minus_test() {
        let mut stack = Stack::new();
        stack.push(Word::new_i64(4)).unwrap();
        stack.push(Word::new_i64(12)).unwrap();

        minus(&mut stack).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap().get_as_i64(), 8);
    }

    #[test]
    fn mp_test() {
        let mut stack = Stack::new();
        stack.push(Word::new_i64(6)).unwrap();
        stack.push(Word::new_i64(-8)).unwrap();

        mp(&mut stack).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap().get_as_i64(), -48);
    }

    #[test]
    fn div_test() {
        let mut stack = Stack::new();
        stack.push(Word::new_i64(7)).unwrap();
        stack.push(Word::new_i64(14)).unwrap();

        div(&mut stack).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap().get_as_i64(), 2);
    }

    #[test]
    fn div_by_zero() {
        let mut stack = Stack::new();
        stack.push(Word::new_i64(0)).unwrap();
        stack.push(Word::new_i64(1337)).unwrap();

        let err = div(&mut stack).unwrap_err();

        assert_eq!(
            err,
            InstError {
                kind: InstErrorKind::DivisionByZero
            }
        )
    }

    #[test]
    fn check_overflow() {
        let mut stack = Stack::new();
        stack.push(Word::new_i64(i64::MAX)).unwrap();
        stack.push(Word::new_i64(2)).unwrap();

        let plus_err = plus(&mut stack).unwrap_err();
        let mp_err = mp(&mut stack).unwrap_err();

        assert_eq!(
            plus_err,
            InstError {
                kind: InstErrorKind::Overflow
            }
        );

        assert_eq!(
            mp_err,
            InstError {
                kind: InstErrorKind::Overflow
            }
        )
    }
}
