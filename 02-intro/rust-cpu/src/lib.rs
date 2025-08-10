use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> f64 {
	let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

	now.as_secs() as f64 + now.subsec_nanos() as f64/1e9
}

pub fn spin(seconds: u64) {
	let start = get_time();
	while (get_time() - start) < seconds as f64 {
		// Busy wait
	}
}