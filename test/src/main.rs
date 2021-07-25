#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_rt::{entry, exception};

static X: AtomicBool = AtomicBool::new(true);

#[inline(never)]
#[entry]
fn main() -> ! {
    foo();

    quux();

    loop {}
}

fn foo() {
    if X.load(Ordering::Relaxed) {
        bar()
    }
}

fn bar() {
    if X.load(Ordering::Relaxed) {
        baz()
    }
}

fn baz() {
    if X.load(Ordering::Relaxed) {
        foo()
    }
}

fn quux() {
    unsafe { llvm_asm!("" : : "r"(0) "r"(1) "r"(2) "r"(3) "r"(4) "r"(5)) }
}

#[exception]
fn SysTick() {
    X.store(false, Ordering::Relaxed);
}