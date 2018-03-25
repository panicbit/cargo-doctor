use failure::{Error, SyncFailure};
use toml_edit;
use cargo_edit;

pub type Result<T> = ::std::result::Result<T, Error>;

pub fn manifest() -> Result<cargo_edit::Manifest> {
    let manifest = cargo_edit::Manifest::open(&None).map_err(SyncFailure::new)?;
    Ok(manifest)
}

pub trait TomlExt {
    fn get_path(&self, path: &[&str]) -> Option<&toml_edit::Item>;
}

impl TomlExt for toml_edit::Item {
    fn get_path(&self, path: &[&str]) -> Option<&toml_edit::Item> {
        path.iter().fold(Some(self), |entry, key| {
            entry
                .and_then(|entry| entry.as_table())
                .and_then(|entry| entry.get(key))
        })
    }
}

impl TomlExt for toml_edit::Document {
    fn get_path(&self, path: &[&str]) -> Option<&toml_edit::Item> {
        self.root.get_path(path)
    }
}

impl TomlExt for cargo_edit::Manifest {
    fn get_path(&self, path: &[&str]) -> Option<&toml_edit::Item> {
        self.data.get_path(path)
    }
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
