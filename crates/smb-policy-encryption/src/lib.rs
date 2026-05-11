//! Encryption Policy — Banking-grade encryption for sensitive SharePoint content

pub mod aes_gcm_kv;

pub trait EncryptionPolicy {
    fn encrypt(&self, data: &[u8], key_id: &str) -> Result<Vec<u8>, String>;
    fn decrypt(&self, ciphertext: &[u8], key_id: &str) -> Result<Vec<u8>, String>;
}