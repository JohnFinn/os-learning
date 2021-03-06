use core::marker::PhantomData;

use common::{IO_BASE, states};
use volatile::prelude::*;
use volatile::{Volatile, WriteVolatile, ReadVolatile, Reserved};

/// An alternative GPIO function.
#[repr(u8)]
pub enum Function {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010
}

#[repr(C)]
#[allow(non_snake_case)]
struct GpioRegisters {
    function_select: [Volatile<u32>; 6],
    __r0: Reserved<u32>,
    set: [WriteVolatile<u32>; 2],
    __r1: Reserved<u32>,
    clear: [WriteVolatile<u32>; 2],
    __r2: Reserved<u32>,
    level: [ReadVolatile<u32>; 2],
    __r3: Reserved<u32>,
    event_detect_status: [Volatile<u32>; 2],
    __r4: Reserved<u32>,
    rising_edge_detect_enable : [Volatile<u32>; 2],
    __r5: Reserved<u32>,
    falling_edge_detect_enable: [Volatile<u32>; 2],
    __r6: Reserved<u32>,
    high_detect_enable: [Volatile<u32>; 2],
    __r7: Reserved<u32>,
    low_detect_enable: [Volatile<u32>; 2],
    __r8: Reserved<u32>,
    async_rising_edge_detect : [Volatile<u32>; 2],
    __r9: Reserved<u32>,
    async_falling_edge_detect_enable: [Volatile<u32>; 2],
    __r10: Reserved<u32>,
    pull_up_down_enable: Volatile<u32>,
    pull_up_down_enable_clock: [Volatile<u32>; 2],
}

/// Possible states for a GPIO pin.
states! {
    Uninitialized, Input, Output, Alt
}

/// A GPIP pin in state `State`.
///
/// The `State` generic always corresponds to an uninstantiatable type that is
/// use solely to mark and track the state of a given GPIO pin. A `Gpio`
/// structure starts in the `Uninitialized` state and must be transitions into
/// one of `Input`, `Output`, or `Alt` via the `into_input`, `into_output`, and
/// `into_alt` methods before it can be used.
pub struct Gpio<State> {
    pin: u8,
    registers: &'static mut GpioRegisters,
    _state: PhantomData<State>
}

/// The base address of the `GPIO` registers.
const GPIO_BASE: usize = IO_BASE + 0x200000;

impl<T> Gpio<T> {
    /// Transitions `self` to state `S`, consuming `self` and returning a new
    /// `Gpio` instance in state `S`. This method should _never_ be exposed to
    /// the public!
    #[inline(always)]
    fn transition<S>(self) -> Gpio<S> {
        Gpio {
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData
        }
    }
}

impl Gpio<Uninitialized> {
    /// Returns a new `GPIO` structure for pin number `pin`.
    ///
    /// # Panics
    ///
    /// Panics if `pin` > `53`.
    pub fn new(pin: u8) -> Gpio<Uninitialized> {
        if pin > 53 {
            panic!("Gpio::new(): pin {} exceeds maximum of 53", pin);
        }

        Gpio {
            registers: unsafe { &mut *(GPIO_BASE as *mut GpioRegisters) },
            pin: pin,
            _state: PhantomData
        }
    }

    /// Enables the alternative function `function` for `self`. Consumes self
    /// and returns a `Gpio` structure in the `Alt` state.
    pub fn into_alt(self, function: Function) -> Gpio<Alt> {
        // 9, 19, 29, 39, 49, 53
        let reg_num = self.pin / 10;
        let offset = (self.pin % 10) * 3;
        self.registers.function_select[reg_num as usize].or_mask((function as u32) << offset);
        self.transition()
    }

    /// Sets this pin to be an _output_ pin. Consumes self and returns a `Gpio`
    /// structure in the `Output` state.
    pub fn into_output(self) -> Gpio<Output> {
        self.into_alt(Function::Output).transition()
    }

    /// Sets this pin to be an _input_ pin. Consumes self and returns a `Gpio`
    /// structure in the `Input` state.
    pub fn into_input(self) -> Gpio<Input> {
        self.into_alt(Function::Input).transition()
    }
}

impl Gpio<Output> {
    /// Sets (turns on) the pin.
    pub fn set(&mut self) {
        let reg_num = self.pin / 32;
        let offset = self.pin - reg_num;
        self.registers.set[reg_num as usize].write(1 << offset);
    }

    /// Clears (turns off) the pin.
    pub fn clear(&mut self) {
        let reg_num = self.pin / 32;
        let offset = self.pin - reg_num;
        self.registers.clear[reg_num as usize].write(1 << offset);
    }
}

impl Gpio<Input> {
    /// Reads the pin's value. Returns `true` if the level is high and `false`
    /// if the level is low.
    pub fn level(&mut self) -> bool {
        let reg_num = self.pin / 32;
        let offset = self.pin - reg_num;
        self.registers.level[reg_num as usize].has_mask(1 << offset)
    }
}
