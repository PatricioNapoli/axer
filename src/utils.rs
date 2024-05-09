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
}
