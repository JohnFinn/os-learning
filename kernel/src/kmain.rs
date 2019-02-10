#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(pointer_methods)]

extern crate pi;
extern crate stack_vec;

use pi::timer::*;
use pi::gpio::*;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut pin = Gpio::new(16);
    // STEP 1: Set GPIO Pin 16 as output.
    let mut pin = pin.into_output();
    // STEP 2: Continuously set and clear GPIO 16.
    loop {
        pin.set();
        spin_sleep_ms(3000);
        pin.clear();
        spin_sleep_ms(500);
    }

    // FIXME: Start the shell.
}
