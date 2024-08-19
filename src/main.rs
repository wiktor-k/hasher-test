fn main() -> testresult::TestResult {
    use sha2::digest::crypto_common::hazmat::SerializableState;
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hello ");

    let result = hasher.serialize();
    eprintln!("{:?}", result);

    let hasher = sha2::Sha256::deserialize(&result)?;
    let result1 = hasher.finalize();
    eprintln!("{:?}", result1);

    let hasher = sha2::Sha256::deserialize(&result)?;
    let result2 = hasher.finalize();
    eprintln!("{:?}", result2);

    assert_eq!(result1, result2);

    Ok(())
}
