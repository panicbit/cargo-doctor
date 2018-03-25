extern crate cargo_edit;
extern crate failure;

#[macro_use]
mod util;
use util::*;

fn main() {
    println!();

    check_manifest_str_field(&["package", "description"], |_| good!("Description is set")).unwrap();
    check_manifest_str_field(&["package", "license"], |license| good!("License: {}", license)).unwrap();
    check_manifest_str_field(&["package", "documentation"], |url| good!("Documentation: {}", url)).unwrap();
    check_manifest_str_field(&["package", "repository"], |url| good!("Repository: {}", url)).unwrap();

    println!();
}

fn check_manifest_str_field<F>(path: &[&str], check: F) -> Result<()>
where
    F: FnOnce(&str)
{
    let manifest = manifest()?;
    let name = path.join(".");

    let value = path.iter().fold(Some(&manifest.data.root), |entry, key| {
        entry
            .and_then(|entry| entry.as_table())
            .and_then(|entry| entry.get(key))
    })
    .map(|entry| entry.as_str());

    match value {
        Some(Some(value)) => check(value),
        Some(None) => bad!("Field '{}' in Cargo.toml is not a string", name),
        None => bad!("Field '{}' in Cargo.toml is not set", name),
    }

    Ok(())
}
