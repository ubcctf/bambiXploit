use crate::config::Config;
use crate::stats::BambiStats;

use std::time::Duration;
use std::{sync::Arc, thread};

pub fn initialize(stats: Arc<BambiStats>, config: Config) {
    let stats_clone = stats.clone();
    thread::spawn(move || {
        log_results(stats_clone, config);
    });
}

fn log_results(stats: Arc<BambiStats>, config: Config) {
    let dur = Duration::from_secs(config.interval as _);
    let mut last = 0;
    loop {
        let now = stats.get_ok();
        let success = 100.0 * (now - last) as f64 / config.addresses.len() as f64;
        println!("Total submitted = {} | % success = {}", now, success);
        last = now;
        thread::sleep(dur);
    }
}
