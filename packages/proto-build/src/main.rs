//! Build Provenance proto files. This build script clones the Provenance version
//! specified in PROVENANCE_REV constant and builds the protos for the provwasm-std library.
//! This is based on the proto-compiler code in github.com/osmosis/osmosis-rust project

use std::{env, path::PathBuf};

use proto_build::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

/// The provenance commit or tag to be cloned and used to build the proto files
const PROVENANCE_REV: &str = "v1.13.1";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../provwasm-std/src/types/";
/// Directory where the provenance submodule is located
const PROVENANCE_DIR: &str = "../../dependencies/provenance/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/provwasm-std/proto-build";

pub fn generate() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(PROVENANCE_DIR, PROVENANCE_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let provenance_project = CosmosProject {
        name: "provenance".to_string(),
        version: PROVENANCE_REV.to_string(),
        project_dir: PROVENANCE_DIR.to_string(),
        include_mods: vec![],
    };

    let provenance_code_generator =
        CodeGenerator::new(out_dir, tmp_build_dir, provenance_project, vec![]);

    provenance_code_generator.generate();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
