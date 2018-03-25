extern crate cargo_edit;
extern crate failure;

#[macro_use]
mod util;
use util::*;

fn main() {
    println!();

    check_license().unwrap();

    println!();
}

fn check_license() -> Result<()> {
    let manifest = manifest()?;

    let license = manifest.data.as_table()
        .get("package").and_then(|f| f.as_table())
        .and_then(|package| package.get("license"))
        .map(|l| l.as_str());

    match license {
        Some(Some(license)) => good!("License is: {}", license),
        Some(None) => bad!("License field in Cargo.toml is not a string"),
        None => bad!("License field in Cargo.toml is not set"),
    }

    Ok(())
}
