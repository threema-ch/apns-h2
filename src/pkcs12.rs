use crate::error::Error;
use p12_keystore::KeyStore;

/// Parse PKCS#12 data, returning a concatenated PEM-encoded certificate chain and PEM-encoded private key.
pub fn parse_pkcs12(pfx_data: &[u8], password: &str) -> Result<(Vec<u8>, Vec<u8>), Error> {
    // Load the keystore
    let ks = KeyStore::from_pkcs12(pfx_data, password).map_err(|_| Error::InvalidCertificate)?;
    // Extract the first private key chain
    let (_alias, chain) = ks.private_key_chain().ok_or(Error::InvalidCertificate)?;
    // Encode certificates as PEM blocks
    let mut cert_pem = Vec::new();
    for cert in chain.chain() {
        let block = pem::Pem::new("CERTIFICATE".to_string(), cert.as_der().to_vec());
        cert_pem.extend(pem::encode(&block).as_bytes());
    }
    // Encode private key as PKCS#8 PEM
    let key_pem_block = pem::Pem::new("PRIVATE KEY".to_string(), chain.key().to_vec());
    let key_pem = pem::encode(&key_pem_block).into_bytes();

    Ok((cert_pem, key_pem))
}
