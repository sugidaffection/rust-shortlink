#[cfg(test)]
mod b62_tests {

    use shortlink::{b62decode, b62encode};

    #[test]
    fn test_base62() {
        let n = 1;
        let encoded = b62encode(n).expect(format!("can't encode {}", n).as_str());
        let decoded = b62decode(encoded.to_owned()).expect("can't decode");
        assert!(decoded == n, "{}", decoded);
    }
}
