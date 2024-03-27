use ao_fec::*;

fn main() {
    const MESSAGE: &[u8] = b"hello";
    let mut packet = [0; encode_len(MESSAGE.len())];
    encode(MESSAGE, &mut packet);
    for p in packet {
        println!("{:02x}", p);
    }
}
