extern crate time;

pub fn record<F>(f: F) -> time::Duration
    where F: FnOnce()
{
    let start = time::now();
    f();
    time::now() - start
}
