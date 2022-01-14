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
