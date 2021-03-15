#![no_std]
#![allow(incomplete_features)]
#![feature(array_map)]
#![feature(new_uninit)]
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
#![feature(const_panic)]

extern crate alloc;

pub mod permutations;
pub mod pruning;
pub mod search;
mod util;
