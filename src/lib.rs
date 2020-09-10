use std::io::Error;
use std::time::Duration;

#[cfg_attr(target_os = "linux", path = "pthread.rs")]
#[cfg_attr(target_os = "macos", path = "mach/mod.rs")]
mod os;

pub use os::clock_for_current_thread;

pub trait OSClock: Sized + Send {
    fn get_time(&self) -> Result<Duration, Error>;
}

#[cfg(test)]
mod tests {
    use super::{clock_for_current_thread, OSClock};

    #[test]
    fn valid_measurement() {
        let clock = clock_for_current_thread().unwrap();

        let mut samples = std::iter::repeat::<()>(())
            .map(|_| clock.get_time().unwrap())
            .step_by(50000);

        let mut last_time = samples.next().unwrap();

        let samples = samples
            .take(5)
            .map(|this_time| {
                assert!(this_time > last_time);
                let diff = (this_time - last_time).as_secs_f64();
                last_time = this_time;
                diff
            })
            .collect::<Vec<f64>>();

        let avg = samples.iter().sum::<f64>() / (samples.len() as f64);

        let mean_abs_dev_scaled = samples
            .iter()
            .map(|sample| (sample - avg).abs())
            .sum::<f64>()
            / (samples.len() as f64)
            / avg;

        println!(
            "
durations of timing 50000 samples
==================================
{:#?}
----------------------------------
avg: {}, mad scaled: {}",
            samples, avg, mean_abs_dev_scaled
        );

        assert!(mean_abs_dev_scaled < 0.1); // test that samples are on average within 10% of the mean
    }
}