# Hasher test

A demo of creating OpenPGP v4 signatures which contain the signed data's digest.

To implement that a state of the digest is captured and restored twice:

1. to recover the original file's digest
2. for appending OpenPGP data for signing purposes.

Note that due to format differences this will *not* work with v6 signatures.

## Running

```sh
$ rm -f Cargo.toml.sig && cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `/home/wiktor/tmp/cargo/debug/hasher-test`
[31, 60, 76, 187, 156, 4, 229, 61, 63, 242, 155, 119, 240, 105, 123, 22, 23, 82, 26, 177, 196, 142, 76, 114, 54, 64, 206, 133, 177, 74, 203, 71, 10, 0, 0, 0, 0, 0, 0, 0, 32, 17, 32, 61, 32, 34, 48, 46, 50, 46, 48, 45, 112, 114, 101, 46, 52, 34, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
Array([112, 192, 230, 102, 32, 49, 255, 58, 36, 245, 11, 201, 85, 55, 107, 153, 42, 48, 127, 87, 51, 236, 212, 106, 19, 49, 169, 110, 245, 199, 15, 241])
Array([112, 192, 230, 102, 32, 49, 255, 58, 36, 245, 11, 201, 85, 55, 107, 153, 42, 48, 127, 87, 51, 236, 212, 106, 19, 49, 169, 110, 245, 199, 15, 241])
```

Inspect the signature:

```sh
$ sq toolbox packet dump Cargo.toml.sig 
Signature Packet, new CTB, 170 bytes
    Version: 4
    Type: Binary
    Pk algo: EdDSA
    Hash algo: SHA256
    Hashed area:
      Signature creation time: 2024-08-19 10:53:04 UTC
      Issuer: 9053327810759706
      Issuer Fingerprint: D7403D0F576A14C676DCDC739053327810759706
      Notation: sha256-hash
        00000000  70 c0 e6 66 20 31 ff 3a  24 f5 0b c9 55 37 6b 99
        00000010  2a 30 7f 57 33 ec d4 6a  13 31 a9 6e f5 c7 0f f1
    Digest prefix: 7612
    Level: 0 (signature over data)
```

Now compare the digest with the file's digest:

```sh
$ sha256sum Cargo.toml
70c0e6662031ff3a24f50bc955376b992a307f5733ecd46a1331a96ef5c70ff1  Cargo.toml
```

Verify the signature:

```sh
$ gpg --import cert.key
$ gpg --verify Cargo.toml.sig 
gpg: assuming signed data in 'Cargo.toml'
gpg: Signature made pon, 19 sie 2024, 12:53:04 CEST
gpg:                using EDDSA key D7403D0F576A14C676DCDC739053327810759706
gpg: Good signature from "testing" [unknown]
gpg: Signature notation: sha256-hash=[ not human readable (32 bytes: p??f 1?:$???U7k?*0?...) ]
gpg: WARNING: This key is not certified with a trusted signature!
gpg:          There is no indication that the signature belongs to the owner.
Primary key fingerprint: 72CA 7FBB 4E80 01F9 6F20  9565 082B 349A 0976 5B4D
     Subkey fingerprint: D740 3D0F 576A 14C6 76DC  DC73 9053 3278 1075 9706
```
