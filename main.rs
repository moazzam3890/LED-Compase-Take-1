#![no_std]
#![no_main]
#![deny(unsafe_code)]

#[allow(unused_imports)]

use aux15::{entry, prelude::*, iprintln, iprint, Direction, I16x3 };

#[entry]

fn main () -> ! {

    let (mut leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();
loop{

    let I16x3 {x, y, ..} = lsm303dlhc.mag().unwrap();
    let mut dir;
    if (x>0) & (y>0) {
        dir = Direction::Southeast;
    }
    else if (x<0) & (y>0) {
        dir = Direction::Northeast;
    }
    else if (x<0) &(y<0) {
        dir = Direction::Northwest;
    }
    else {
        dir = Direction::Southwest;
    }

    leds.iter_mut().for_each(|led| led.off());
    leds[dir].on();
    iprintln!(&mut itm.stim[0],"{:?}", lsm303dlhc.mag().unwrap());

    delay.delay_ms(1_000_u16);
}
}