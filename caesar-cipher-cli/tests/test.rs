#[cfg(test)]
mod tests {
    use caeser_cipher_cli::{decrypt, encrypt, to_all_caps};
    
    #[test]
    fn test_encrypt_decrypt() {
        let text = "Harry Xia";
        let shift = 3;
        let encrypted = encrypt(text, shift);
        let decrypted = decrypt(&encrypted, shift);
        
        assert_eq!(text, decrypted);
    }

    #[test]
    fn test_to_all_caps() {
        assert_eq!(to_all_caps("hello"), "HELLO");
        assert_eq!(to_all_caps("WORLD"), "WORLD");
        assert_eq!(to_all_caps("MixedCase"), "MIXEDCASE");
        assert_eq!(to_all_caps(""), "");
    }
}