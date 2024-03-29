fn main() {
    let bindings = bindgen::Builder::default()
        .allowlist_function("(ao_fec_encode|ao_fec_decode)")
        .header("ao-fec/src/ao_fec.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("ao_fec_ffi.rs"))
        .expect("could not write bindings");

    cc::Build::new()
        .file("ao-fec/src/ao_fec_tx.c")
        .file("ao-fec/src/ao_fec_rx.c")
        .compile("ao_fec");
}
