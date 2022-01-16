use std::env::var;
use std::ffi::OsStr;
use std::str::FromStr;

pub fn or<K: AsRef<OsStr>, T: FromStr>(k: K, default: T) -> T {
    let v = var(k);

    v.ok().and_then(|s| s.parse::<T>().ok()).unwrap_or(default)
}
