#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for &value in values {
        let mut value_bytes = Vec::new();
        let mut remaining = value;
        while remaining >= 0x80 {
            value_bytes.push((remaining & 0x7F) as u8 | 0x80);
            remaining >>= 7;
        }
        value_bytes.push(remaining as u8);
        bytes.extend(value_bytes);
    }
    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut numbers = Vec::new();
    let mut value = 0;
    for &byte in bytes {
        value = (value << 7) | (byte & 0x7F) as u32;
        if byte & 0x80 == 0 {
            numbers.push(value);
            value = 0;
        }
    }
    if value != 0 {
        Err(Error::IncompleteNumber)
    } else {
        Ok(numbers)
    }
}
