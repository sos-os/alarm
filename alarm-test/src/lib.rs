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
#[address_repr(u8)]
pub struct MockAddress(u8);

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
        MockAddress(self.frame[0])
    }

    fn end_address(&self) -> Self::Address {
        MockAddress(self.frame[0])
    }

    fn number(&self) -> usize {
        &self.number as *const _ as usize
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_fn_works_alarm_test() {
        assert_eq!(2 + 2, 4);
    }
}
