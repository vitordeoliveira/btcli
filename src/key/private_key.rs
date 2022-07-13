use num::BigUint;

use crate::key::constants::N;
use crate::key::Key;
use crate::utils::ToByteArray;

#[derive(Debug, PartialEq)]
pub enum PrivateKeyError {
    GreaterThanCurveOrder,
    InvalidSize,
    InvalidHex(hex::FromHexError),
}

impl From<hex::FromHexError> for PrivateKeyError {
    fn from(err: hex::FromHexError) -> Self {
        PrivateKeyError::InvalidHex(err)
    }
}

/// A struct representing Secp256k1 private key
///
/// "The private key can be any number between 0 and n - 1, inclusive, where n is a constant
/// defined as the order of the Secp256k1 elliptic curve."
///
/// n = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141
#[derive(Debug, PartialEq)]
pub struct PrivateKey {
    pub key: Vec<u8>,
}

impl PrivateKey {
    /// Returns a private key struct given it's bytes.
    ///
    /// # Arguments
    ///
    /// * `privkey` - Private key as a string slice of hexadecimals digits.
    pub fn from_str(privkey_as_str: &str) -> Result<Self, PrivateKeyError> {
        if privkey_as_str.len() > 64 {
            return Err(PrivateKeyError::InvalidSize);
        }

        let mut privkey_as_str = privkey_as_str.to_string();

        if privkey_as_str.len() < 64 {
            privkey_as_str = format!("{:0>width$}", privkey_as_str, width = 64);
        }

        let key = Vec::from_str(&privkey_as_str)?;

        let less_than_curve_order = key < N.to_string().to_byte_array().unwrap();

        match less_than_curve_order {
            true => Ok(PrivateKey { key }),
            false => Err(PrivateKeyError::GreaterThanCurveOrder),
        }
    }

    /// Returns a hexadecimal string representing the private key
    fn as_hex_string(&mut self) -> String {
        self.key.as_hex_string()
    }

    /// Returns a hexadecimal string representing the "compressed" private key.
    fn as_hex_compressed_string(mut self) -> String {
        self.key.push(0x01);

        self.key.as_hex_string()
    }

    /// Returns a bs58 encoded string representing the private key in the WIF format.
    fn as_wif(&mut self) -> String {
        self.key.insert(0, 0x80);
        self.key.append_checksum();

        bs58::encode(&self.key).into_string()
    }

    /// Returns a bs58 encoded string representing the private key in the WIF-compressed format.
    fn as_wif_compressed(&mut self) -> String {
        self.key.insert(0, 0x80);
        self.key.push(0x01);
        self.key.append_checksum();

        bs58::encode(&self.key).into_string()
    }

    /// Returns the private key as decimal string
    fn as_decimals(self) -> String {
        format!("{}", BigUint::from_bytes_be(&self.key))
    }
}

#[cfg(test)]
mod private_key_tests {
    use super::{PrivateKey, PrivateKeyError};
    use crate::key::constants::{COMPRESSED_PRIVATE_KEY, COMPRESSED_WIF, N, PRIVATE_KEY, WIF};

    #[test]
    fn constructor_should_return_private_key() {
        let mut pk = PrivateKey::from_str(PRIVATE_KEY).unwrap();

        assert_eq!(pk.as_hex_string(), PRIVATE_KEY.to_string())
    }

    #[test]
    fn should_pad_if_input_is_not_32_bytes() {
        let mut pk = PrivateKey::from_str("123").unwrap();
        let expected = "0000000000000000000000000000000000000000000000000000000000000123";

        assert_eq!(pk.as_hex_string(), expected);
    }

    #[test]
    fn should_throw_error_if_invalid_hex_digits() {
        let c = 'v';
        let index = 63;

        assert_eq!(
            PrivateKey::from_str("v"),
            Err(PrivateKeyError::InvalidHex(
                hex::FromHexError::InvalidHexCharacter { c, index }
            ))
        );
        assert_eq!(
            PrivateKey::from_str(
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD036414v"
            ),
            Err(PrivateKeyError::InvalidHex(
                hex::FromHexError::InvalidHexCharacter { c, index }
            ))
        );
    }

    #[test]
    fn should_throw_error_if_input_is_greater_than_curve_order() {
        assert_eq!(
            PrivateKey::from_str(N),
            Err(PrivateKeyError::GreaterThanCurveOrder)
        );
        assert_eq!(
            PrivateKey::from_str(
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364142"
            ),
            Err(PrivateKeyError::GreaterThanCurveOrder)
        )
    }

    #[test]
    fn should_append_compressed_suffix() {
        assert_eq!(
            PrivateKey::from_str(PRIVATE_KEY)
                .unwrap()
                .as_hex_compressed_string(),
            COMPRESSED_PRIVATE_KEY,
        )
    }

    #[test]
    fn should_return_expected_wif_format() {
        assert_eq!(PrivateKey::from_str(PRIVATE_KEY).unwrap().as_wif(), WIF,)
    }

    #[test]
    fn should_return_expected_wif_compressed_format() {
        assert_eq!(
            PrivateKey::from_str(PRIVATE_KEY).unwrap().as_wif_compressed(),
            COMPRESSED_WIF
        )
    }

    #[test]
    fn should_throw_error_if_input_is_greater_than_64_digits() {
        let pk = PrivateKey::from_str("1e99423a4ed27608a15a2616a2b0e9e52ced330ac530edcc32c8ffc6a526aeddd");

        assert_eq!(
            pk,
            Err(PrivateKeyError::InvalidSize),
        )
    }

    #[test]
    fn should_convert_to_expected_decimal_string() {
        let pk = PrivateKey::from_str(PRIVATE_KEY).unwrap();
        let expected = "13840170145645816737842251482747434280357113762558403558088249138233286766301";

        assert_eq!(
            pk.as_decimals(),
            expected,
        )
    }
}
