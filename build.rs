fn main() {
    let status = std::process::Command::new("make")
        .current_dir("ao-fec/src")
        .status()
        .expect("make failed to run");
    assert!(status.success(), "make failed");
    println!("cargo::rustc-link-search=ao-fec/src");
    println!("cargo::rustc-link-lib=ao_fec");
}
