use ao_fec::*;

fn main() {
    let mut packet = Vec::new();
    encode(b"hello", &mut packet);
    for p in packet {
        println!("{:02x}", p);
    }
}
