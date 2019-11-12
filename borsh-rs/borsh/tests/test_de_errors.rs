use borsh::BorshDeserialize;

#[derive(BorshDeserialize, Debug)]
enum A {
    X,
    Y,
}

#[derive(BorshDeserialize, Debug)]
struct B {
    x: u64,
    y: u32,
}

#[test]
fn test_missing_bytes() {
    let bytes = vec![1, 0];
    assert_eq!(
        B::try_from_slice(&bytes).unwrap_err().to_string(),
        "failed to fill whole buffer"
    );
}

#[test]
fn test_invalid_enum_variant() {
    let bytes = vec![123];
    assert_eq!(
        A::try_from_slice(&bytes).unwrap_err().to_string(),
        "Unexpected variant index: 123"
    );
}

#[test]
fn test_extra_bytes() {
    let bytes = vec![1, 0, 0, 0, 32, 32];
    assert_eq!(
        <Vec<u8>>::try_from_slice(&bytes).unwrap_err().to_string(),
        "Not all bytes read"
    );
}

#[test]
fn test_invalid_bool() {
    for i in 2u8..=255 {
        let bytes = [i];
        assert_eq!(
            <bool>::try_from_slice(&bytes).unwrap_err().to_string(),
            format!("Invalid bool representation: {}", i)
        );
    }
}

#[test]
fn test_invalid_option() {
    for i in 2u8..=255 {
        let bytes = [i, 32];
        assert_eq!(
            <Option<u8>>::try_from_slice(&bytes)
                .unwrap_err()
                .to_string(),
            format!(
                "Invalid Option representation: {}. The first byte must be 0 or 1",
                i
            )
        );
    }
}

#[test]
fn test_invalid_result() {
    for i in 2u8..=255 {
        let bytes = [i, 0];
        assert_eq!(
            <Result<u64, String>>::try_from_slice(&bytes)
                .unwrap_err()
                .to_string(),
            format!(
                "Invalid Result representation: {}. The first byte must be 0 or 1",
                i
            )
        );
    }
}

#[test]
fn test_invalid_length() {
    let bytes = vec![255u8; 4];
    assert_eq!(
        <Vec<u64>>::try_from_slice(&bytes).unwrap_err().to_string(),
        "failed to fill whole buffer"
    );
}

#[test]
fn test_invalid_length_string() {
    let bytes = vec![255u8; 4];
    assert_eq!(
        String::try_from_slice(&bytes).unwrap_err().to_string(),
        "failed to fill whole buffer"
    );
}

#[test]
fn test_non_utf_string() {
    let bytes = vec![1, 0, 0, 0, 0xC0];
    assert_eq!(
        String::try_from_slice(&bytes).unwrap_err().to_string(),
        "invalid utf-8 sequence of 1 bytes from index 0"
    );
}

#[test]
fn test_nan_float() {
    let bytes = vec![0, 0, 192, 127];
    assert_eq!(
        f32::try_from_slice(&bytes).unwrap_err().to_string(),
        "For portability reasons we do not allow to deserialize NaNs."
    );
}

#[test]
fn test_evil_bytes() {
    // test takes a really long time if read() is used instead of read_exact()
    let bytes = vec![255, 255, 255, 255];
    assert_eq!(
        <Vec<[u8; 32]>>::try_from_slice(&bytes)
            .unwrap_err()
            .to_string(),
        "error deserializing element at index 0: failed to fill whole buffer"
    );
}
