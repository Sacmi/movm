use crate::word::Word;
use std::fmt;

pub const STACK_MAX_SIZE: usize = 90;

#[derive(Debug, PartialEq)]
pub enum StackErrorKind {
    Overflow,
    Underflow,
    Index,
}

#[derive(Debug, PartialEq)]
pub struct StackError {
    pub(super) kind: StackErrorKind,
}

impl StackError {
    pub fn __description(&self) -> &str {
        match self.kind {
            StackErrorKind::Overflow => "stack is full",
            StackErrorKind::Underflow => "stack is empty",
            StackErrorKind::Index => "index out of range",
        }
    }
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

pub struct Stack {
    stack: [Word; STACK_MAX_SIZE],
    current_size: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; STACK_MAX_SIZE],
            current_size: 0,
        }
    }

    pub fn at(&self, index: usize) -> Result<Word, StackError> {
        if index >= self.current_size {
            Err(StackError {
                kind: StackErrorKind::Index,
            })
        } else {
            Ok(self.stack[index])
        }
    }

    pub fn push(&mut self, value: Word) -> Result<(), StackError> {
        if self.current_size == STACK_MAX_SIZE {
            return Err(StackError {
                kind: StackErrorKind::Overflow,
            });
        }

        self.stack[self.current_size] = value;
        self.current_size += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<Word, StackError> {
        if self.current_size == 0 {
            return Err(StackError {
                kind: StackErrorKind::Underflow,
            });
        }

        self.current_size -= 1;
        Ok(self.stack[self.current_size])
    }

    pub fn dump(&self) {
        println!("Dump of stack:");

        for i in 0..self.current_size {
            println!(" - {} -> {}", i, self.stack[i])
        }
    }

    pub fn get_size(&self) -> usize {
        self.current_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_stack() {
        let stack = Stack::new();

        assert_eq!(stack.current_size, 0);
        assert_eq!(stack.stack, [0; STACK_MAX_SIZE])
    }

    #[test]
    fn push() {
        let mut stack = Stack::new();

        stack.push(1).unwrap();
        stack.push(-1).unwrap();

        assert_eq!(stack.current_size, 2);

        let mut arr: [Word; STACK_MAX_SIZE] = [0; STACK_MAX_SIZE];
        arr[0] = 1;
        arr[1] = -1;

        assert_eq!(stack.stack, arr);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();

        stack.push(10).unwrap();
        stack.push(20).unwrap();

        assert_eq!(stack.pop().unwrap(), 20);
        assert_eq!(stack.current_size, 1);

        assert_eq!(stack.pop().unwrap(), 10);
        assert_eq!(stack.current_size, 0)
    }

    #[test]
    fn overflow() {
        let mut stack = Stack::new();

        for i in 0..STACK_MAX_SIZE {
            stack.push(i as Word).unwrap();
        }

        let err = stack.push(1);
        assert_eq!(
            err.unwrap_err(),
            StackError {
                kind: StackErrorKind::Overflow
            }
        )
    }

    #[test]
    fn underflow() {
        let mut stack = Stack::new();
        let err = stack.pop();

        assert_eq!(
            err.unwrap_err(),
            StackError {
                kind: StackErrorKind::Underflow
            }
        )
    }
}
