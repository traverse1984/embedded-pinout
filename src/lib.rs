#![no_std]

#[cfg(feature = "rp2040")]
#[macro_use]
mod rp2040;

#[cfg(feature = "rppal")]
#[macro_use]
mod rppal;
