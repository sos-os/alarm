extern crate alarm_base;
extern crate hal9000;
#[macro_use]
extern crate hal9000_derive;
extern crate core;

use core::{cmp, ops};
use hal9000::{
    mem::{Address, Page},
    util::{self, Align},
};


#[derive(Address)]
#[address_repr(usize)]
pub struct MockAddress(usize);

struct MockFrame {
    number: usize,
    frame: [u8; 4096],
}

impl Page for MockFrame {
    const SHIFT: usize = 0;
    const SIZE: usize = 4096;
    type Address = MockAddress;

    fn from_addr_up(addr: Self::Address) -> Self {
        unimplemented!()
    }

    fn from_addr_down(addr: Self::Address) -> Self {
        unimplemented!()
    }

    fn base_address(&self) -> Self::Address {
        MockAddress(&self.frame[0] as *const _ as usize)
    }

    fn end_address(&self) -> Self::Address {
        MockAddress(&self.frame[Self::SIZE - 1] as *const _ as usize)
    }

    fn number(&self) -> usize {
        self.number
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_fn_works_alarm_test() {
        use super::*;

        let f = MockFrame { number: 0, frame: [0; 4096]};

        println!("{:?}", f.base_address());
        println!("{:?}", f.end_address());


    }
}
