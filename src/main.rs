#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103_hal as hal;
use hal::prelude::*;
use hal::pac as pac;

extern "C" {
    fn enable_mcycle_minstret();
    // fn disable_mcycle_minstret();
    // fn trap_entry();
    // fn irq_entry();
}

#[entry]
fn main() -> ! {
    unsafe { enable_mcycle_minstret() };
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    pa1.set_low().unwrap();
    loop {}
}
