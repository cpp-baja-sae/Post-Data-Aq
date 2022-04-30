const MASKS: [u64; 7] = [
    0b___1_101_1010101_101010101010101_10101010101010101010101010101010000001,
    0b___1_011_0110011_011001100110011_01100110011001100110011001100110000010,
    0b___0_111_0001111_000111100001111_00011110000111100001111000011110000100,
    0b___0_000_1111111_000000011111111_00000001111111100000000111111110001000,
    0b___0_000_0000000_111111111111111_00000000000000011111111111111110010000,
    0b___0_000_0000000_000000000000000_11111111111111111111111111111110100000,
    0b___1_111_1111111_111111111111111_11111111111111111111111111111111111111,
];

/// Returns true if `of` has an odd number of ones or false if an even number.
fn parity(of: u64) -> bool {
    let of: u32 = ((of >> 32) ^ of) as u32;
    let of = (of >> 16) ^ of;
    let of = (of >> 8) ^ of;
    let of = (of >> 4) ^ of;
    let of = (of >> 2) ^ of;
    let of = (of >> 1) ^ of;
    of & 0x1 == 1
}

pub fn encode_u64(data: u64) -> u64 {
    assert!(data & 0b111_1111 == 0);
    let mut data = data;
    for mask in 0..MASKS.len() {
        data |= if parity(data & MASKS[mask]) { 1 } else { 0 } << mask;
    }
    data
}

pub fn encode_bytes(data: &mut [u8]) {
    if let Ok(data_as_u64s) = bytemuck::try_cast_slice_mut(data) {
        for item in data_as_u64s {
            *item = encode_u64(*item);
        }
    } else {
        panic!("Invalid data size!");
    }
}

pub fn decode_u64(data: u64) -> Result<u64, ()> {
    // All 1s, tells us where the error is.
    let mut error_mask = 0xFFFF_FFFF_FFFF_FFFF;
    let mut encountered_any_error = false;
    for mask in 0..MASKS.len() {
        let mask = MASKS[mask];
        if parity(data & mask) {
            encountered_any_error = true;
            error_mask &= mask;
        } else {
            error_mask &= !mask;
        }
    }

    if encountered_any_error && error_mask == 0 {
        // We know an error happened, but we don't know where!
        Err(())
    } else {
        Ok((data ^ error_mask) & !0b111_1111)
    }
}

pub fn decode_bytes(data: &mut [u8]) -> Result<(), ()> {
    if let Ok(data_as_u64s) = bytemuck::try_cast_slice_mut(data) {
        for item in data_as_u64s {
            *item = decode_u64(*item)?;
        }
        Ok(())
    } else {
        panic!("Invalid data size!")
    }
}

#[test]
fn test_hamming_code() {
    let data = 0x1234_5678_9ABC_DE_00;
    let encoded = encode_u64(data);
    assert_eq!(decode_u64(encoded), Ok(data));
    for index in 0..63 {
        let decoded = decode_u64(encoded ^ (1 << index));
        assert_eq!(decoded, Ok(data));
    }
    assert_eq!(decode_u64(encoded | 0b11), Err(()));
}
