use sdk::TargetOptions;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;
use toml::Value as Toml;

error_chain!{
    foreign_links {
        Io(::std::io::Error);
    }
}

/*
* Load workspace Cargo.toml
* Load target crate workspace.toml
* for each crate in workspace, look for BUILD.gn file
* List-em
*/

#[derive(Debug, Deserialize)]
struct Manifest {
    package: Option<Package>,
    dependencies: Option<Toml>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: Option<String>,
}

pub fn get_dependency_names(manifest: &str) -> Result<HashSet<String>> {
    let decoded: Manifest = toml::from_str(&manifest).unwrap();
    let deps = decoded.dependencies.chain_err(|| "Crate manifest had no dependencies.")?;
    let mut dep_set = HashSet::new();
    let deps_table = match deps {
        Toml::Table(table) => table,
        _ => bail!("Crate manifest dependencies not a table"),
    };
    for (key, value) in deps_table.iter() {
        match value {
            &Toml::String(ref version) => {
                dep_set.insert(key.clone());
            }
            _ => bail!("Crate {} manifest has a non-string dependency", key),
        }
    }
    Ok(dep_set)
}

pub fn list_gn_deps(target_options: &TargetOptions, crate_path: &PathBuf) -> Result<()> {
    let full_path = crate_path.canonicalize()?;
    println!("target_options = {:?}, full_path = {:?}", target_options, full_path);
    let cargo_toml_path = full_path.join("Cargo.toml");
    println!("cargo_toml_path = {:?}", cargo_toml_path);
    let mut cargo_toml_file = File::open(cargo_toml_path)?;
    let mut toml_str = String::new();
    cargo_toml_file.read_to_string(&mut toml_str)?;

    let dep_names = get_dependency_names(&toml_str)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    static FUCHSIA_APP_CONTENTS: &'static str = r#"
    # Copyright 2017 The Fuchsia Authors. All rights reserved.
    # Use of this source code is governed by a BSD-style license that can be
    # found in the LICENSE file.

    [package]
    name = "fuchsia-app"
    version = "0.1.0"
    license = "BSD-3-Clause"
    authors = ["Taylor Cramer <cramertj@google.com>"]
    description = "Library for managing Fuchsia applications and services"

    [dependencies]
    fdio = "0.2.0"
    fidl = "0.1.0"
    fuchsia-zircon = "0.3.2"
    futures = "0.1.15"
    garnet_examples_fidl_services = "0.1.0"
    garnet_public_lib_app_fidl = "0.1.0"
    garnet_public_lib_app_fidl_service_provider = "0.1.0"
    mxruntime = "0.1.0"
    tokio-core = "0.1"
    tokio-fuchsia = "0.1.0"
    "#;

    use gn_deps::get_dependency_names;

    #[test]
    fn test_get_dependency_names() {
        let result = get_dependency_names(FUCHSIA_APP_CONTENTS).unwrap();
        println!("result = {:?}", result);
        assert_eq!(10, result.len());
    }

}