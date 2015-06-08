use std::process::Command;
use std::env;

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let arg = format!("OUTDIR={}", outdir);

    let output = Command::new("make").arg("-C").arg("c_bits").arg(arg).output().unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if ! output.status.success() {
        panic!("Didn't successfully make!: {}", stderr);
    }
    
    println!("cargo:rustc-link-search=native={}", outdir);
    println!("cargo:rustc-link-lib=static=startup")
}