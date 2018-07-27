#![feature(stdsimd)]
#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate panic_abort;

use core::arch::arm;

use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let x = 42;

    loop {
        unsafe { arm::__NOP() }
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

#[inline(always)]
fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {
        unsafe { arm::__NOP() }
    }
}

// define the default exception handler
exception!(*, default_handler);

#[inline(always)]
fn default_handler(_irqn: i16) {}
