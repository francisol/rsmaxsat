use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use flate2::read::GzDecoder;
use tar::Archive;

const  EVALMAXSAT_URL:&str ="https://github.com/FlorentAvellaneda/EvalMaxSAT/archive/refs/heads/master.tar.gz";

fn get_extract_dir(out_dir: &PathBuf) -> PathBuf {
    fs::read_dir(&out_dir)
        .expect("Failed to read extracted directory")
        .filter_map(Result::ok)
        .find(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .map(|entry| entry.path())
        .expect("Failed to find extracted solver directory")
}

fn download_and_extract(url: &str, name: &str) -> PathBuf {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_dir = out_path.join("third_parts").join(name);
    let out_dir_ref = &out_dir;
    let complete_file = out_dir_ref.join(".complete");
    if complete_file.exists() {
        return get_extract_dir(out_dir_ref);
    }
    let response = reqwest::blocking::get(url).expect("Failed to download solver");
    let tar = GzDecoder::new(response);
    let mut archive = Archive::new(tar);
    fs::create_dir_all(out_dir_ref).expect("Failed to create directory");
    archive
        .unpack(out_dir_ref)
        .expect("Failed to extract solver");
    // Find the extracted directory (it might have a version suffix)
    let d = get_extract_dir(out_dir_ref);
    apply_patch(&d, name);
    fs::write(&complete_file, b"").expect("Failed to create .complete file");
    d
}

fn binding_evalmaxsat() {
    let evalmaxsat_dir = download_and_extract(EVALMAXSAT_URL, "evalmaxsat");

    let dst = cmake::Config::new(&evalmaxsat_dir).build_target("EvalMaxSAT").build();
    println!("cargo:rustc-link-search={}/build/lib/EvalMaxSAT", dst.display());
    println!("cargo:rustc-link-search={}/build/lib/cadical", dst.display());//MaLib
    println!("cargo:rustc-link-search={}/build/lib/MaLib", dst.display());//MaLib
    println!("cargo:rustc-link-lib=static=EvalMaxSAT");
    println!("cargo:rustc-link-lib=static=cadical");
    println!("cargo:rustc-link-lib=static=MaLib");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed={}", evalmaxsat_dir.display());

    let bindings = bindgen::Builder::default()
        .headers([format!(
            "{}/lib/EvalMaxSAT/src/CadicalEvalMaxSAT.hpp",
            evalmaxsat_dir.display()
        )])
        .allowlist_function("Eval::.*")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("evalmaxsat_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn apply_patch(dir: &PathBuf, patch_name: &str) {
    let patch_path = PathBuf::from("patches").join(format!("{}.patch", patch_name));
    if patch_path.exists() {
        let abs_path = fs::canonicalize(&patch_path).unwrap();
        let status = Command::new("patch")
            .arg("-f")
            .arg("-r")
            .arg(".ignore")
            .arg("--ignore-whitespace")
            .arg("--directory")
            .arg(dir)
            .arg("-p1")
            .arg("-i")
            .arg(abs_path)
            .status()
            .expect("Failed to execute git apply");
        if !status.success() {
            println!("cargo::warning=Failed to apply patch: {}", patch_name);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if cfg!(feature = "evalmaxsat") {
        binding_evalmaxsat();
    }

}
