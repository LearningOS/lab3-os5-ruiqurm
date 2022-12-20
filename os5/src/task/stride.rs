use core::cmp::Ordering;

use crate::config::HALF_BIG_STRIDE;

pub struct Pass(usize);

impl Pass{
    pub fn new() -> Self{
        Self(0)
    }

    pub fn increase(&mut self,stride:usize){
        self.0 += stride;
    }
}

impl PartialOrd for Pass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let diff : usize;
        if self.0 > other.0 {
            diff = self.0 - other.0;
        } else{
            diff = other.0 - self.0;
        }
        if diff <= HALF_BIG_STRIDE{
            Some(self.0.cmp(&other.0))
        } else {
            Some(other.0.cmp(&self.0))
        } 
    }
}

impl PartialEq for Pass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}