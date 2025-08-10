use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub fn get_time() -> f64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    now.as_secs() as f64 + now.subsec_nanos() as f64 / 1e9
}

#[allow(dead_code)]
pub fn spin(howlong: u64) {
    let start = get_time();
    while (get_time() - start) < howlong as f64 {
        // do nothing in loop
    }
}
