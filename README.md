# Playfair-Cipher
A very experimental implementation of the playfair cipher. This is being used as a playground to try out different Rust patterns
and ways of implementing the same thing.

## Usage

The library is exposing two methods:

* `encrypt(key: &str, plaintext: &str)`
* `decrypt(key: &str, cipher: &str)`

The test cases show how it can be used:

```rust
#[test]
fn encrypt_plaintext() {
    let plaintext = "instruments";
    let key = "monarchy";
    let encrypted_string = encrypt(key, plaintext);
    assert_eq!(
        encrypted_string,
        "gatlmzclrqtx".to_string().to_ascii_uppercase()
    );
}

#[test]
fn decrypt_plaintext() {
    let ciphertext = "gatlmzclrqtx";
    let key = "monarchy";
    let decrypted_string = decrypt(key, ciphertext);
    assert_eq!(
        decrypted_string,
        "instrumentsz".to_string().to_ascii_uppercase()
    );
}
```
