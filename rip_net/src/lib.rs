#![feature(async_await, async_closure)]

pub mod error;
pub mod peer;
pub mod codec;

#[macro_use]
extern crate futures;
extern crate bytes;