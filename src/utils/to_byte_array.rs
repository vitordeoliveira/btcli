pub trait ToByteArray {
    fn to_byte_array(self) -> Vec<u8>;
}

impl ToByteArray for String {
    fn to_byte_array(self) -> Vec<u8> {
        hex::decode(self).unwrap()
    }
}

#[cfg(test)]
mod to_byte_array_tests {
    use super::ToByteArray;

    fn assert_eq(input: &str, expected: Vec<u8>) {
        assert_eq!(
            input.to_string().to_byte_array(),
            expected,
        )
    }

    #[test]
    fn should_return_expected_byte_array() {
        assert_eq("00", vec![0x00]);
        assert_eq("05", vec![0x05]);
        assert_eq("6f", vec![0x6f]);
        assert_eq("80", vec![0x80]);
        assert_eq("0142", vec![0x01, 0x42]);
        assert_eq("0488b21e", vec![0x04, 0x88, 0xb2, 0x1e]);
    }

}
