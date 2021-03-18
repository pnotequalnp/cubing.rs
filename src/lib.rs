#![no_std]
#![allow(incomplete_features)]
#![feature(array_map)]
#![feature(array_zip)]
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
#![feature(const_panic)]
#![feature(iter_intersperse)]

#[macro_use]
extern crate alloc;

pub mod core;
pub mod kociemba;
pub mod rubiks;
pub mod util;

pub use crate::core::*;
