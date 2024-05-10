use sha2::Digest;
use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum Error {
    #[error("overflow error")]
    OverflowError,
}

/// Convert a byte array to a u64.
/// Overflow is checked and returns an error.
pub fn byte_array_to_u64(slice: &[u8]) -> Result<u64, Error> {
    let mut num: u64 = 0;

    for byte in slice.iter().rev() {
        num = num
            .checked_mul(256)
            .ok_or(Error::OverflowError)?
            .checked_add(*byte as u64)
            .ok_or(Error::OverflowError)?;
    }

    Ok(num)
}

pub fn sha256(message: &[u8]) -> [u8; 32] {
    let mut context = sha2::Sha256::new();
    context.update(message);
    let mut result: [u8; 32] = [0; 32];
    result.copy_from_slice(context.finalize().as_ref());
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Error::OverflowError;

    #[test]
    fn test_byte_array_to_u64() {
        let bytes = [
            100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        assert_eq!(byte_array_to_u64(&bytes), Ok(100));

        let bytes = [
            255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(byte_array_to_u64(&bytes), Ok(u64::MAX));

        let bytes = [
            255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(byte_array_to_u64(&bytes), Err(OverflowError));
    }

    #[test]
    fn test_sha256() {
        let message = b"hello world";
        let expected = [
            185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218, 125, 171, 250, 196, 132, 239,
            227, 122, 83, 128, 238, 144, 136, 247, 172, 226, 239, 205, 233,
        ];
        assert_eq!(sha256(message), expected);
    }
}
