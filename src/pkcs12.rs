use crate::error::Error;
use p12_keystore::KeyStore;

/// Parse PKCS#12 data, returning a concatenated PEM-encoded certificate chain and PEM-encoded private key.
pub fn parse_pkcs12(pfx_data: &[u8], password: &str) -> Result<(Vec<u8>, Vec<u8>), Error> {
    // Load the keystore
    let keystore = KeyStore::from_pkcs12(pfx_data, password).map_err(|_| Error::InvalidCertificate)?;

    // Extract the first private key chain
    let (_alias, private_key_chain) = keystore.private_key_chain().ok_or(Error::InvalidCertificate)?;

    // Encode certificates as PEM blocks
    let cert_pem = {
        let mut cert_pem = Vec::new();

        for cert in private_key_chain.chain() {
            let block = pem::Pem::new("CERTIFICATE", cert.as_der().to_vec());
            cert_pem.extend(pem::encode(&block).as_bytes());
        }

        cert_pem
    };

    // Encode private key as PKCS#8 PEM
    let key_pem = {
        let key_pem_block = pem::Pem::new("PRIVATE KEY", private_key_chain.key().to_vec());
        pem::encode(&key_pem_block).into_bytes()
    };

    Ok((cert_pem, key_pem))
}
