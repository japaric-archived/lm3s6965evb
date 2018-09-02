#![feature(stdsimd)]
#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

#[cfg(feature = "semihosting")]
extern crate cortex_m_semihosting as semihosting;

extern crate panic_abort;

use core::arch::arm;

use rt::ExceptionFrame;

#[cfg(feature = "semihosting")]
use core::fmt::Write;

entry!(main);

fn main() -> ! {
    let x = 42;

    loop {
        unsafe { arm::__NOP() }

        #[cfg(feature = "semihosting")]
        unsafe {
            let mut hstdout = semihosting::hio::hstdout().unwrap();
            write!(hstdout, "x = {}\n", x);

            // system call to exit successfully
            semihosting::syscall1(
                semihosting::nr::REPORT_EXCEPTION,
                semihosting::debug::Exception::ApplicationExit as usize,
            );
        }
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
