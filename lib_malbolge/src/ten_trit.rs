use core::panic;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TenTrit(u16);

impl TenTrit {
    pub const MIN: TenTrit = TenTrit(0);
    pub const MAX: TenTrit = TenTrit(59_048);

    pub fn new(x: u16) -> Self {
        if x <= Self::MAX.0 {
            TenTrit(x)
        } else {
            panic!("Out of Range")
        }
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}
impl Add for TenTrit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let total = self.0 + rhs.0;
        if total > Self::MAX.0 {
            TenTrit(total % Self::MAX.0)
        } else {
            TenTrit(total)
        }
    }
}

impl From<TenTrit> for usize {
    fn from(value: TenTrit) -> Self {
        value.0.into()
    }
}

impl Sub for TenTrit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.0 > self.0 {
            println!("TUVEQUE HACER ESTO");
            TenTrit(rhs.0 - self.0)
        } else {
            TenTrit(self.0 - rhs.0)
        }
    }
}
impl Default for TenTrit {
    fn default() -> Self {
        TenTrit::MIN
    }
}
