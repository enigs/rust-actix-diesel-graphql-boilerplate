use anyhow::Result;
use xsalsa20poly1305::aead::{ Aead, KeyInit };
use xsalsa20poly1305::aead::generic_array::{ GenericArray, typenum };
use xsalsa20poly1305::XSalsa20Poly1305;

const NONCE_LENGTH: usize = 24;

pub fn decrypt<T>(content: T) -> Result<String>
    where T: ToString
{
    // Create bindings and then generate cipher instance
    let bindings = base64_url::decode(&std::env::var("MASTER_KEY")?)?;
    let key = GenericArray::from_slice(&bindings);
    let cipher = XSalsa20Poly1305::new(key);

    // Set content
    let content = base64_url::decode(&content.to_string())?;
    if content.len() <= NONCE_LENGTH {
        return Err(anyhow::anyhow!("Invalid content length"));
    }

    // Split content
    let (nonce, content) = content.split_at(NONCE_LENGTH);

    // Set nonce & content
    let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);
    let content = match cipher.decrypt(nonce, content) {
        Ok(content) => content,
        Err(_) => return Err(anyhow::anyhow!("Unable to decrypt content"))
    };

    // Return decrypted content
    Ok(String::from_utf8_lossy(&content).to_string())
}

pub fn encrypt<T>(content: T) -> Result<String>
    where T: ToString
{
    // Create bindings and then generate cipher instance
    let bindings = base64_url::decode(&std::env::var("MASTER_KEY")?)?;
    let key = GenericArray::from_slice(&bindings);
    let cipher = XSalsa20Poly1305::new(key);

    // Set nonce
    let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
    let content = match cipher.encrypt(&nonce, content.to_string().as_bytes()) {
        Ok(content) => content,
        Err(_) => return Err(anyhow::anyhow!("Unable to encrypt content"))
    };

    // Return encrypted content
    Ok(base64_url::encode(&[&nonce[..], &content[..]].concat()))
}