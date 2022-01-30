use secded::SecDedCodec;

type HammingCode = secded::SecDed64;

/// Modifies a stream of bytes to include Hamming error correction codes. Every
/// eighth byte must be zero.
pub fn encode(input: &mut [u8]) {
    debug_assert_eq!(
        input.len() % 8,
        0,
        "input size must be a multiple of 8 bytes."
    );
    for chunk in input.chunks(8) {
        debug_assert_eq!(chunk[7], 0, "every eighth byte must be zero.");
    }
    let hc = HammingCode::new(57);
    hc.encode(input)
}

pub fn decode(input: &mut [u8]) -> Result<(), ()> {
    debug_assert_eq!(
        input.len() % 8,
        0,
        "input size must be a multiple of 8 bytes."
    );
    let hc = HammingCode::new(57);
    hc.decode(input)
}
