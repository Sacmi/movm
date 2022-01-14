use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub union Word {
    as_i64: i64,
    as_u64: u64,
    as_f64: f64,
}

impl Word {
    pub fn zero() -> Word {
        Word { as_u64: 0 }
    }

    pub fn new_i64(value: i64) -> Word {
        Word { as_i64: value }
    }

    pub fn new_u64(value: u64) -> Word {
        Word { as_u64: value }
    }

    pub fn new_f64(value: f64) -> Word {
        Word { as_f64: value }
    }

    pub fn get_as_i64(&self) -> i64 {
        unsafe { self.as_i64 }
    }

    pub fn get_as_u64(&self) -> u64 {
        unsafe { self.as_u64 }
    }

    pub fn get_as_f64(&self) -> f64 {
        unsafe { self.as_f64 }
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.as_u64 == other.as_u64 }
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("Word")
                .field("i64", &self.as_i64)
                .field("u64", &self.as_u64)
                .field("f64", &self.as_f64)
                .finish()
        }
    }
}
