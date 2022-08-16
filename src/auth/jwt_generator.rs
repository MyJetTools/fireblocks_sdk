use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use sha2::{Sha256, Digest};


use super::FireblocksClaims;

pub fn generate_fireblocs_jwt(path: &str, api_key: String, pem: Vec<u8>, body: String) -> String{
    let body_hash = get_body_hash(body);

    let claims = FireblocksClaims::new(&path,&api_key, &body_hash);

    let header = Header::new(Algorithm::RS256);
    let key = EncodingKey::from_rsa_pem(&pem).unwrap();
    let token_str = jsonwebtoken::encode(&header, &claims, &key);

    return token_str.unwrap();
}

pub fn generate_fireblocs_jwt_without_body(path: &str, api_key: String, pem: Vec<u8>) -> String{    
    let body_hash = get_body_hash("\"\"".into());

    let claims = FireblocksClaims::new(&path,&api_key, &body_hash);

    let header = Header::new(Algorithm::RS256);
    let key = EncodingKey::from_rsa_pem(&pem).unwrap();
    
    let token_str = encode(&header, &claims, &key);

    return token_str.unwrap();
}

fn get_body_hash(body: String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(body.clone());
    let data = hasher.finalize();
    hex::encode(data)
}
