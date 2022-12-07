use std::collections::HashSet;

use jsonwebtoken::{DecodingKey, Algorithm, Validation, errors::Error, EncodingKey, Header};

use crate::{CoSignerCallback, CoSignerCallbackResponse};

pub fn decrypt_co_signer_request(request: &str, public_key: Vec<u8>) -> Result<CoSignerCallback, Error>{
    let key = DecodingKey::from_rsa_pem(&public_key)?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.required_spec_claims = HashSet::new();

    let result = jsonwebtoken::decode::<CoSignerCallback>(&request, &key, &validation)?;

    return Ok(result.claims);   
}

pub fn encrypt_co_signer_request(response: CoSignerCallbackResponse, private_key: Vec<u8>) -> Result<String, Error>{
    let key = EncodingKey::from_rsa_pem(&private_key)?;
    let result = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &response, &key)?;
    return Ok(result);   
}