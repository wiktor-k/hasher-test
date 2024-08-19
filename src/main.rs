use pgp::packet::*;
use pgp::types::*;
use pgp::Deserializable as _;
use pgp::SignedSecretKey;
use sha2::digest::crypto_common::hazmat::SerializableState;
use sha2::Digest;

fn main() -> testresult::TestResult {
    let mut hasher = sha2::Sha256::new();
    hasher.update(std::fs::read("Cargo.toml")?);

    let result: &[u8] = &hasher.serialize();
    eprintln!("{:?}", result);

    let hasher = sha2::Sha256::deserialize(&result.try_into()?)?;
    let result1 = hasher.finalize();
    eprintln!("{:?}", result1);

    let hasher = sha2::Sha256::deserialize(&result.try_into()?)?;
    let result2 = hasher.finalize();
    eprintln!("{:?}", result2);

    let signer = SignedSecretKey::from_bytes(std::fs::File::open("cert.pgp")?)?;

    let sig = calculate_signature(&signer.secret_subkeys[0], &result)?;

    assert_eq!(result1, result2);

    let mut f = std::fs::File::create("Cargo.toml.sig")?;
    //sig.to_writer(&mut f)?;
    pgp::packet::write_packet(&mut f, &sig)?;

    Ok(())
}

/// Set up a signature packet, hash `data`, and make a cryptographic signature with `signer`
fn calculate_signature(
    signer: impl SecretKeyTrait,
    state: &[u8],
) -> Result<Signature, Box<dyn std::error::Error>> {
    let hasher = sha2::Sha256::deserialize(&state.try_into()?)?;
    let file_hash = hasher.finalize();

    let mut sig_config =
        SignatureConfig::v4(SignatureType::Binary, signer.algorithm(), signer.hash_alg());
    sig_config.hashed_subpackets = vec![
        Subpacket::regular(SubpacketData::SignatureCreationTime(
            std::time::SystemTime::now().into(),
        )),
        Subpacket::regular(SubpacketData::Issuer(signer.key_id())),
        Subpacket::regular(SubpacketData::IssuerFingerprint(
            //KeyVersion::V4,
            signer.fingerprint().into(),
        )),
        Subpacket::regular(SubpacketData::Notation(Notation {
            readable: false,
            name: "sha256-hash".into(),
            value: file_hash[..].into(),
        })),
    ];

    //let mut hasher = sig_config.hash_alg.new_hasher()?;

    //sig_config.hash_data_to_sign(&mut *hasher, data)?;
    let mut hasher = sha2::Sha256::deserialize(&state.try_into()?)?;
    let write: &mut dyn std::io::Write = &mut hasher;

    let len = sig_config.hash_signature_data(write)?;
    hasher.update(&sig_config.trailer(len)?);

    let hash = &hasher.finalize()[..];

    let signed_hash_value = [hash[0], hash[1]];
    let raw_sig = signer.create_signature(String::new, sig_config.hash_alg, hash)?;

    let signature = Signature::from_config(sig_config, signed_hash_value, raw_sig);

    Ok(signature)
}
