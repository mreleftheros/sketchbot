use std::{thread, time};

pub fn sleep_s(secs: u64) {
  thread::sleep(time::Duration::from_secs(secs));
}

pub fn sleep_ms(ms: u64) {
  thread::sleep(time::Duration::from_millis(ms));
}