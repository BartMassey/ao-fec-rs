use ao_fec::*;

fn main() {
    const MESSAGE: &[u8] = b"hello";
    const EL: usize = encode_len(MESSAGE.len());
    let mut packet = [0; EL];
    encode(MESSAGE, &mut packet);

    let mut received = [0; 8 * EL];
    for (i, byte) in packet.into_iter().enumerate() {
        for j in 0..8 {
            received[8 * i + j] = 0xff * (1 - ((byte >> (7 - j)) & 1));
        }
    }
    

    let mut decoded = [0; MESSAGE.len() + 2];
    let decoded = decode(&received, &mut decoded).unwrap();
    assert_eq!(decoded, MESSAGE);
}
