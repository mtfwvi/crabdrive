/// Convert a &[u8] into a human-readable HEX string
pub fn to_hex_str(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let vec = bytes
        .iter()
        .flat_map(|&b| [HEX[(b >> 4) as usize], HEX[(b & 0x0f) as usize]])
        .collect();
    String::from_utf8(vec).expect("Failed to convert")
}
