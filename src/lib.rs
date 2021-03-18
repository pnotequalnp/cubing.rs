#![no_std]
#![allow(incomplete_features)]
#![feature(array_map)]
#![feature(array_zip)]
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
#![feature(const_panic)]

#[macro_use]
extern crate alloc;

pub mod definitions;
pub mod pruning;
pub mod search;
pub mod transition;
mod util;
