use libc::{clockid_t, pthread_self, pthread_t};
use std::io::Error;
use std::os::raw::c_int;

use super::PosixClock;

#[link(name = "pthread")]
extern "C" {
    fn pthread_getcpuclockid(thread: pthread_t, clock_id: *mut clockid_t) -> c_int;
}

/// A type alias for compatibility with Mach
pub type ThreadCPUClock = PosixClock;

/// Get a clock for the CPU time of the current thread
///
/// ```
/// use std::io;
/// use os_clock::{self, Clock};
///
/// let clock = os_clock::cpu_clock_for_current_thread().unwrap();
/// let time = clock.get_time().unwrap();
///
/// # let time_2 = clock.get_time().unwrap();
/// #
/// # assert!(time_2 > time);
/// ```
pub fn cpu_clock_for_current_thread() -> Result<ThreadCPUClock, Error> {
    let mut clockid = 0 as clockid_t;

    // unsafe because accessing FFI, which doesnt change global state, fills clock_id
    if unsafe { pthread_getcpuclockid(pthread_self(), &mut clockid) } != 0 {
        return Err(Error::last_os_error());
    }

    // valid because clockid is known to be valid
    let clock = unsafe { ThreadCPUClock::from_clockid(clockid) };

    return Ok(clock);
}
