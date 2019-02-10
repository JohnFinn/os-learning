use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct TimerRegisters {
    control_status: Volatile<u32>,
    counter_lower: ReadVolatile<u32>,
    counter_higher: ReadVolatile<u32>,
    compare: [Volatile<u32>; 4]
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut TimerRegisters
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut TimerRegisters) },
        }
    }

    /// Reads the system timer's counter and returns the 64-bit counter value.
    /// The returned value is the number of elapsed microseconds.
    pub fn read(&self) -> u64 {
        let mut result: u64 = (self.registers.counter_higher.read() as u64) << 32;
        result | self.registers.counter_lower.read() as u64
    }
}

/// Returns the current time in microseconds.
pub fn current_time() -> u64 {
    Timer::new().read()
}

/// Spins until `us` microseconds have passed.
pub fn spin_sleep_us(us: u64) {
    let timer = Timer::new();
    let now = timer.read() + us;
    timer.registers.compare[0].write(now as u32);
    while timer.registers.control_status.read() & 1 == 0 {
        
    }
}

/// Spins until `ms` milliseconds have passed.
pub fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms*1000);
}
