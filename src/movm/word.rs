#[derive(Copy, Clone)]
pub union Word {
    pub as_i64: i64,
    pub as_u64: u64,
    pub as_f64: f64,
}

impl Word {
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
