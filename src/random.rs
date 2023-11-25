use std::time::{SystemTime, UNIX_EPOCH};

static mut STATE: u64 = 2023;

pub fn seed_current_time() {
    unsafe {
        STATE = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

pub fn rand() -> f64 {
    f64::from_bits(0x3ff << 52 | wyrand() >> 12) - 1.0
}

pub fn rand_between(min: f64, max: f64) -> f64 {
    min + rand() * (max - min)
}

// adapted from https://github.com/lemire/testingRNG/blob/master/source/wyrand.h
fn wyrand() -> u64 {
    let t;
    unsafe {
        STATE = STATE.wrapping_add(0xa0761d6478bd642f);
        t = (STATE as u128) * ((STATE ^ 0xe7037ed1a0b428db) as u128);
    }
    ((t >> 64) as u64) ^ (t as u64)
}
