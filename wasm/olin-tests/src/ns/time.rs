extern crate chrono;
extern crate olin;

use self::chrono::TimeZone;
use log::info;

/// This tests for https://github.com/CommonWA/cwa-spec/blob/master/ns/time.md
pub extern "C" fn test() -> Result<(), i32> {
    info!("running ns::time tests");

    let now: i64 = olin::time::ts();
    let dt = chrono::Utc.timestamp(now, 0);

    info!("ts: {}, dt: {}", now, dt.to_rfc3339());

    let now = olin::time::now();
    info!("time::now(): {}", now.to_rfc3339());

    info!("ns::time tests passed");
    Ok(())
}
