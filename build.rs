fn main() {
    cc::Build::new()
        .file("ao-fec/src/ao_fec_tx.c")
        .file("ao-fec/src/ao_fec_rx.c")
        .compile("ao_fec");
}
