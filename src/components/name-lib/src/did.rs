

use jsonwebtoken::{jwk::Jwk, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use async_trait::async_trait;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, engine::general_purpose::STANDARD,Engine as _};
use crate::{NSError, NSResult,decode_jwt_claim_without_verify};
#[derive(Clone, Serialize, Deserialize,Debug,PartialEq)]
pub struct DID {
    pub method: String,
    pub id: String,
}

pub const DID_DOC_AUTHKEY: &str = "#auth-key";

impl DID {
    pub fn new(method: &str, id: &str) -> Self {
        DID {
            method: method.to_string(),
            id: id.to_string(),
        }
    }

    pub fn get_auth_key(&self) -> Option<[u8; 32]> {
        if self.method == "dev" {
            let auth_key = URL_SAFE_NO_PAD.decode(self.id.as_str()).unwrap();
            return Some(auth_key.try_into().unwrap());
        }
        None
    }
    
    pub fn from_str(did: &str) -> Option<Self> {
        let parts: Vec<&str> = did.split(':').collect();
        if parts.len() != 3 {
            return None;
        }
        if parts[0] != "did" {
            return None;
        }
        Some(DID {
            method: parts[1].to_string(),
            id: parts[2].to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("did:{}:{}", self.method, self.id)
    }

    pub fn to_host_name(&self) -> String {
        format!("{}.{}.did",self.id,self.method)
    }

    pub fn from_host_name(host_name: &str) -> Option<Self> {
        let parts: Vec<&str> = host_name.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        if parts[2] != "did" {
            return None;
        }
        Some(DID {
            id: parts[0].to_string(),
            method: parts[1].to_string(),
        })
    }
}



#[derive(Clone, Serialize, Deserialize,Debug,PartialEq)]
pub enum EncodedDocument {
    JsonLd(Value),
    Jwt(String),
}

impl EncodedDocument {
    pub fn to_string(&self) -> String {
        match self {
            EncodedDocument::Jwt(jwt) => jwt.clone(),
            EncodedDocument::JsonLd(value) => serde_json::to_string(value).unwrap(),
        }
    }

    pub fn from_str(doc_str: String) -> NSResult<Self> {
        if doc_str.starts_with("{") {
            let real_value = serde_json::from_str(&doc_str)
                .map_err(|e| NSError::DecodeJWTError(e.to_string()))?;
            return Ok(EncodedDocument::JsonLd(real_value));
        }
        return Ok(EncodedDocument::Jwt(doc_str));
    }

    pub fn to_json_value(self)->NSResult<Value> {
        match self {
            EncodedDocument::Jwt(jwt_str) => {
                let claims = decode_jwt_claim_without_verify(jwt_str.as_str())
                    .map_err(|e| NSError::DecodeJWTError(e.to_string()))?;
                Ok(claims)
            },
            EncodedDocument::JsonLd(value) => Ok(value),
        }
    }
}

#[async_trait]
pub trait DIDDocumentTrait {
    fn get_did(&self) -> &str;
    fn get_auth_key(&self) -> Option<DecodingKey>;
    fn is_proof(self) -> bool;
    fn get_prover_kid(&self) -> Option<String>;
    fn get_iss(&self) -> Option<String>;
    fn get_exp(&self) -> Option<u64>;
    fn get_iat(&self) -> Option<u64>;

    fn encode(&self,key:Option<&EncodingKey>) -> NSResult<EncodedDocument>;
    fn decode(doc: &EncodedDocument,key:Option<&DecodingKey>) -> NSResult<Self> where Self: Sized;
    // async fn decode_with_load_key<'a, F, Fut>(doc: &'a EncodedDocument,loader:F) -> NSResult<Self> 
    //     where Self: Sized,
    //           F: Fn(&'a str) -> Fut,
    //           Fut: std::future::Future<Output = NSResult<DecodingKey>>;

    //JSON-LD
    //fn to_json_value(&self) -> Value;
    //fn from_json_value(value: &Value) -> Self;
}



// #[derive(Clone, Serialize, Deserialize)]
// pub struct DIDDocument<T> {
//     pub did: String,
//     pub payload: T, 
//     pub auth_key: Option<Jwk>,
//     pub iss:Option<String>,
//     pub exp:u64,
//     pub iat:Option<u64>,
// }

