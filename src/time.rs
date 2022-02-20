use std::time::Duration;
use tokio::time::{sleep, Sleep};

pub fn my_sleep(ms: u64) -> Sleep {
	sleep(Duration::from_millis(ms))
}
