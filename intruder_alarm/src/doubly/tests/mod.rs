//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2015-2017 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//

use super::*;
use super::Linked;
use quickcheck::TestResult;
use std::default::Default;

#[derive(Default, Debug)]
pub struct NumberedNode {
    pub number: usize,
    links: Links<NumberedNode>,
}

impl NumberedNode {
    pub fn new(number: usize) -> Self {
        NumberedNode {
            number: number,
            ..Default::default()
        }
    }
}

impl Linked for NumberedNode {
    #[inline]
    fn links(&self) -> &Links<Self> {
        &self.links
    }
    #[inline]
    fn links_mut(&mut self) -> &mut Links<Self> {
        &mut self.links
    }
}
impl PartialEq for NumberedNode {
    fn eq(&self, rhs: &Self) -> bool {
        self.number == rhs.number
    }
}

impl From<usize> for NumberedNode {
    fn from(u: usize) -> NumberedNode {
        NumberedNode::new(u)
    }
}

impl Into<usize> for NumberedNode {
    fn into(self) -> usize {
        self.number
    }
}

mod boxed;
mod mut_ref;