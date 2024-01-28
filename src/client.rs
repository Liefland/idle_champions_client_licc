use crate::{ApiKey, Code, InsertCodeRequest, Source};
use reqwest;
use std::collections::HashMap;

static DEFAULT_BASE_URL: &str = "https://codes.idlechampions.liefland.net/v1/";

pub struct CodesClient {
    base_url: String,
    api_key: Option<ApiKey>,
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum ClientError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    ApiKeyMissing,
}

#[derive(serde::Deserialize)]
struct RetrieveCodesCodeResponse {
    code: String,
    expired: bool,
    expires_at: String,
    sources: SourcesMapping,
}

#[derive(serde::Deserialize)]
struct SourcesMapping {
    creator: i32,
    submitter: i32,
    lister: i32,
}

#[derive(serde::Deserialize)]
struct RetrieveCodesResponse {
    codes: Vec<RetrieveCodesCodeResponse>,
    sources: HashMap<i32, Source>,
}

impl CodesClient {
    pub fn new(client: reqwest::Client, base_url: String, api_key: Option<ApiKey>) -> Self {
        Self {
            base_url,
            api_key,
            client,
        }
    }

    pub fn default_with_api_key(api_key: ApiKey) -> Self {
        Self {
            api_key: Some(api_key),
            ..Default::default()
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn get(&self, url: &str) -> Result<String, ClientError> {
        let response = self
            .client
            .get(self.url(url))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(ClientError::Reqwest)?;

        response.text().await.map_err(ClientError::Reqwest)
    }

    pub async fn put(&mut self, url: &str, body: &str) -> Result<String, ClientError> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or(ClientError::ApiKeyMissing)?
            .get();

        let response = self
            .client
            .put(self.url(url))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Api-Key", api_key)
            .body(body.to_string())
            .send()
            .await
            .map_err(ClientError::Reqwest)?;

        response.text().await.map_err(ClientError::Reqwest)
    }

    pub async fn get_codes(&self) -> Result<Vec<Code>, ClientError> {
        let response = self.get("codes").await?;

        let codes: RetrieveCodesResponse = serde_json::from_str(&response).unwrap();

        Ok(mapping_full(codes))
    }

    pub async fn get_codes_slim(&self) -> Result<Vec<Code>, ClientError> {
        let response = self.get("codes").await?;

        let codes: RetrieveCodesResponse = serde_json::from_str(&response).unwrap();

        Ok(mapping_slim(codes))
    }

    // pub async fn insert_code(
    //     &mut self,
    //     _insert_request: InsertCodeRequest,
    // ) -> Result<i32, ClientError> {
    //     Ok(0)
    // }
}

impl Default for CodesClient {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: None,
            client: reqwest::Client::new(),
        }
    }
}

fn mapping_slim(codes: RetrieveCodesResponse) -> Vec<Code> {
    codes
        .codes
        .into_iter()
        .map(|code| Code {
            code: code.code,
            expired: code.expired,
            expires_at: None,
            creator: None,
            submitter: None,
            lister: None,
        })
        .collect::<Vec<Code>>()
}

fn mapping_full(codes: RetrieveCodesResponse) -> Vec<Code> {
    codes
        .codes
        .into_iter()
        .map(|code| {
            let creator = codes.sources.get(&code.sources.creator).cloned();
            let submitter = codes.sources.get(&code.sources.submitter).cloned();
            let lister = codes.sources.get(&code.sources.lister).cloned();

            Code {
                code: code.code,
                expired: code.expired,
                expires_at: Some(code.expires_at),
                creator,
                submitter,
                lister,
            }
        })
        .collect::<Vec<Code>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construct_client_default() {
        assert!(CodesClient::default().api_key.is_none());
    }

    #[test]
    fn test_construct_client_with_api_key() {
        assert!(
            CodesClient::default_with_api_key(ApiKey::new("foo".to_string()))
                .api_key
                .is_some()
        );
    }

    #[test]
    fn test_client_url() {
        let client = CodesClient::default();
        assert_eq!(client.url("foo"), format!("{}foo", DEFAULT_BASE_URL));
    }

    #[test]
    fn test_mapping_slim() {
        let m = mapping_slim(mock_response());
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].code, "FOOB-BARS-TEST");
        assert!(!m[0].expired);
        assert!(m[0].expires_at.is_none());
        assert!(m[0].creator.is_none());
        assert!(m[0].submitter.is_none());
        assert!(m[0].lister.is_none());
    }

    #[test]
    fn test_mapping_full() {
        let m = mapping_full(mock_response());
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].code, "FOOB-BARS-TEST");
        assert!(!m[0].expired);
        assert!(m[0].expires_at.is_some());
        assert_eq!(m[0].expires_at, Some("2024-01-01 00:00:00.0".to_string()));
        assert!(m[0].creator.is_some());
        assert!(m[0].submitter.is_some());
        assert!(m[0].lister.is_some());
    }

    fn mock_response() -> RetrieveCodesResponse {
        let mut sources = HashMap::new();
        sources.insert(
            1,
            Source {
                id: 1,
                name: "foo".to_string(),
                url: "https://foo.example".to_string(),
            },
        );
        sources.insert(
            2,
            Source {
                id: 2,
                name: "bar".to_string(),
                url: "https://bar.example".to_string(),
            },
        );
        sources.insert(
            3,
            Source {
                id: 3,
                name: "lister".to_string(),
                url: "https://lister.example".to_string(),
            },
        );

        RetrieveCodesResponse {
            codes: vec![RetrieveCodesCodeResponse {
                code: "FOOB-BARS-TEST".to_string(),
                expired: false,
                expires_at: "2024-01-01 00:00:00.0".to_string(),
                sources: SourcesMapping {
                    creator: 1,
                    submitter: 2,
                    lister: 3,
                },
            }],
            sources,
        }
    }
}
