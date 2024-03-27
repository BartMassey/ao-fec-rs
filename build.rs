fn main() {
    let status = std::process::Command::new("make")
        .current_dir("csrc")
        .status()
        .expect("make failed to run");
    assert!(status.success(), "make failed");
    println!("cargo::rustc-link-search=csrc");
    println!("cargo::rustc-link-lib=ao_fec");
}
