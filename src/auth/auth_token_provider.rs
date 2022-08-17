use super::{generate_fireblocs_jwt, generate_fireblocs_jwt_without_body};

pub trait ApiTokenProvider {
    fn get_api_key(&self) -> String;
    fn get_private_key(&self) -> Vec<u8>;
    fn sign_jwt(&self, path: &str, body: Option<String>) -> String;
}

#[derive(Clone)]
pub struct BaseApiTokenProvider{
    pub api_key: String,
    pub private_key: Vec<u8>,
}

impl BaseApiTokenProvider {
    pub fn new(api_key: String, path_to_secret: String) -> Self{
        let rsa_pem = std::fs::read(&path_to_secret).unwrap();

        Self { api_key: api_key, private_key: rsa_pem }
    }
}

impl ApiTokenProvider for BaseApiTokenProvider {
    fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    fn get_private_key(&self) -> Vec<u8> {
        self.private_key.clone()
    }

    fn sign_jwt(&self, path: &str, body: Option<String>) -> String {
        match body {
            Some(body) => generate_fireblocs_jwt(path, self.api_key.clone(), self.private_key.clone(), body),
            None => generate_fireblocs_jwt_without_body(path, self.api_key.clone(), self.private_key.clone()),
        }
    }
}