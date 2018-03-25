use cargo_edit::Manifest;
use failure::{Error, SyncFailure};

pub type Result<T> = ::std::result::Result<T, Error>;

pub fn manifest() -> Result<Manifest> {
    let manifest = Manifest::open(&None).map_err(SyncFailure::new)?;
    Ok(manifest)
}

macro_rules! good {
    () => (good!(""));
    ($fmt:expr) => (good!($fmt,));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("  ✔  ", $fmt), $($arg)*));
}

macro_rules! bad {
    () => (bad!(""));
    ($fmt:expr) => (bad!($fmt,));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("  ✘  ", $fmt), $($arg)*));
}

macro_rules! warn {
    () => (warn!(""));
    ($fmt:expr) => (warn!($fmt,));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("  ⚠  ", $fmt), $($arg)*));
}