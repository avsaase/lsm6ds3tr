#![no_std]
#![allow(dead_code)]

pub mod consts;
pub mod interface;
pub mod lsm6ds3tr;
pub mod measurements;
pub mod registers;
pub mod settings;

pub use lsm6ds3tr::*;
