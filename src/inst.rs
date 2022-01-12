use std::fmt;

use crate::stack::{Stack, Word};
use crate::VM;

#[derive(Debug, PartialEq)]
pub enum InstErrorKind {
    NotEnoughOperands,
    DivisionByZero,
    StackOverflow,
    IllegalPointer,
    IndexOutOfRange,
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
            InstErrorKind::IndexOutOfRange => "index out of range"
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

#[derive(Debug, Copy, Clone)]
pub struct Inst {
    pub typ: InstType,
    pub op: Word,
}

macro_rules! check_operands {
    ($a:expr, $b:expr) => {
        if $a.get_size() < $b {
            return Err(InstError { kind: InstErrorKind::NotEnoughOperands });
        }
    };
}

pub fn push(stack: &mut Stack, op: Word) -> Result<(), InstError> {
    let res = stack.push(op);

    if res.is_ok() {
        Ok(())
    } else {
        Err(InstError { kind: InstErrorKind::StackOverflow })
    }
}

pub fn plus(stack: &mut Stack) -> Result<(), InstError> {
    check_operands!(stack, 2);

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a + b).unwrap();

    Ok(())
}

pub fn minus(stack: &mut Stack) -> Result<(), InstError> {
    check_operands!(stack, 2);

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a - b).unwrap();

    Ok(())
}

pub fn mp(stack: &mut Stack) -> Result<(), InstError> {
    check_operands!(stack, 2);

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a * b).unwrap();

    Ok(())
}

pub fn div(stack: &mut Stack) -> Result<(), InstError> {
    check_operands!(stack, 2);

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    if b == 0 {
        return Err(InstError { kind: InstErrorKind::DivisionByZero });
    }

    stack.push(a / b).unwrap();
    Ok(())
}

pub fn dump(stack: &mut Stack) -> Result<(), InstError> {
    stack.dump();
    Ok(())
}

pub fn jmp(vm: &mut VM, op: Word) -> Result<(), InstError> {
    if op > vm.get_program_length() as Word || op < 0 {
        return Err(InstError { kind: InstErrorKind::IllegalPointer });
    }

    vm.set_inst_pointer(op as usize);

    Ok(())
}

pub fn dup(stack: &mut Stack, op: Word) -> Result<(), InstError> {
    if op >= stack.get_size() as Word || op < 0 {
        return Err(InstError { kind: InstErrorKind::IndexOutOfRange });
    }

    let duplicate = stack.at(stack.get_size() - 1 - (op as usize)).unwrap();
    let res = stack.push(duplicate);

    if res.is_err() {
        Err(InstError { kind: InstErrorKind::StackOverflow })
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

        push(&mut stack, 69).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap(), 69);
    }

    #[test]
    fn plus_test() {
        let mut stack = Stack::new();

        stack.push(15).unwrap();
        stack.push(20).unwrap();

        plus(&mut stack).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap(), 35);
    }

    #[test]
    fn macro_test() {
        let mut stack = Stack::new();
        let err = plus(&mut stack);

        assert_eq!(err.unwrap_err(), InstError { kind: InstErrorKind::NotEnoughOperands })
    }

    #[test]
    fn minus_test() {
        let mut stack = Stack::new();
        stack.push(4).unwrap();
        stack.push(12).unwrap();

        minus(&mut stack).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap(), 8);
    }

    #[test]
    fn mp_test() {
        let mut stack = Stack::new();
        stack.push(6).unwrap();
        stack.push(-8).unwrap();

        mp(&mut stack).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap(), -48);
    }

    #[test]
    fn div_test() {
        let mut stack = Stack::new();
        stack.push(7).unwrap();
        stack.push(14).unwrap();

        div(&mut stack).unwrap();

        assert_eq!(stack.get_size(), 1);
        assert_eq!(stack.pop().unwrap(), 2);
    }

    #[test]
    fn div_by_zero() {
        let mut stack = Stack::new();
        stack.push(0).unwrap();
        stack.push(1337).unwrap();

        let err = div(&mut stack).unwrap_err();

        assert_eq!(err, InstError { kind: InstErrorKind::DivisionByZero })
    }
}