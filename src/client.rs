use crate::api_key::ApiKey;
use crate::client::error::{ClientError, ErrorResponse};
#[cfg(feature = "write")]
use crate::write;
use crate::{Code, Source};
use reqwest;
use std::collections::HashMap;

/// The default base URL
/// This points to the service hosted by the author of this crate.
static DEFAULT_BASE_URL: &str = "https://codes.idlechampions.liefland.net/v1";

pub struct CodesClient {
    base_url: String,
    #[allow(dead_code)]
    api_key: Option<ApiKey>,
    client: reqwest::Client,
}

pub mod error {
    /// Any error that can happen during a request
    #[derive(Debug)]
    pub enum ClientError {
        /// Reqwest error
        Reqwest(reqwest::Error),
        /// Request failed to serialize or Response failed to deserialize
        Serde(serde_json::Error),
        /// The remote has returned a non-successful HTTP status code
        ServerError(ErrorResponse),
        /// You are attempting to make a write request without an API Key
        #[cfg(feature = "write")]
        ApiKeyMissing,
    }

    /// ErrorResponse is returned from the remote when an error occurs.
    /// Does not happen in most read scenarios.
    #[derive(Debug, serde::Deserialize)]
    pub struct ErrorResponse {
        pub error: InnerErrorResponse,
    }

    /// Object inside of an ErrorResponse
    #[derive(Debug, serde::Deserialize)]
    pub struct InnerErrorResponse {
        /// The status code of the error (maps to the HTTP status code in most cases)
        pub code: i32,
        /// The error message
        pub description: String,
        /// If the remote allows listing of debug messages, this will be populated
        /// It will give concrete context of what went wrong on the remote
        pub debug: Option<String>,
    }
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

#[cfg(feature = "write")]
mod puts {
    use crate::write::InsertCodeRequest;

    #[derive(Clone, Debug, serde::Serialize)]
    pub(crate) struct RemoteInsertCodeRequest {
        code: String,
        expires_at: u64,
        creator_name: String,
        creator_url: String,
        submitter_name: Option<String>,
        submitter_url: Option<String>,
    }

    impl From<InsertCodeRequest> for RemoteInsertCodeRequest {
        fn from(value: InsertCodeRequest) -> Self {
            let (submitter_name, submitter_url) = match value.submitter {
                None => (None, None),
                Some(s) => (Some(s.name), Some(s.url)),
            };

            Self {
                code: value.code,
                expires_at: value.expires_at,
                creator_name: value.creator.name,
                creator_url: value.creator.url,
                submitter_name,
                submitter_url,
            }
        }
    }
}

impl CodesClient {
    /// Construct a new CodesClient providing an optional API key
    /// If no values need to ever change - or if the `write` feature is always disabled,
    /// consider using `default` instead.
    pub fn new(api_key: Option<ApiKey>) -> Self {
        Self::new_full(api_key, None, None)
    }

    /// Construct a new CodesClient, optionally providing an API Key.
    /// If left to None, default values will be used.
    /// If no values need to change, consider using `default` instead.
    pub fn new_full(
        api_key: Option<ApiKey>,
        base_url: Option<String>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            client: client.unwrap_or_else(Self::default_client),
            api_key,
        }
    }

    fn url(&self, path: &str) -> String {
        if path.starts_with('/') {
            return format!("{}{}", self.base_url, path);
        }
        format!("{}/{}", self.base_url, path)
    }

    /// Perform any arbitrary GET request and take ownership of deserializing the response.
    pub async fn get(&self, route: &str) -> Result<String, ClientError> {
        let response = self
            .client
            .get(self.url(route))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(ClientError::Reqwest)?;

        self.response(response).await
    }

    #[cfg(feature = "write")]
    /// Perform any arbitrary PUT request and take ownership of deserializing the response.
    /// These actions typically require an API key.
    pub async fn put(&mut self, route: &str, body: &str) -> Result<String, ClientError> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or(ClientError::ApiKeyMissing)?
            .get();

        let response = self
            .client
            .put(self.url(route))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Api-Key", api_key)
            .body(body.to_string())
            .send()
            .await
            .map_err(ClientError::Reqwest)?;

        self.response(response).await
    }

    /// Query HTTP GET `/api/v1/codes` and deserialize the response.
    ///
    /// This is useful if you need the code itself, and the meta-information.
    /// All Optional fields will try to have values, unless they were not provided by the remote.
    ///
    /// If you only need the code and the expiry information, use `get_codes_slim` instead.
    pub async fn get_codes(&self) -> Result<Vec<Code>, ClientError> {
        let response = self.get("/codes").await?;

        let codes: RetrieveCodesResponse = serde_json::from_str(&response).unwrap();

        Ok(mapping_full(codes))
    }

    /// Query HTTP GET `/api/v1/codes` and deserialize the response, returning a slim subset including only essential data.
    ///
    /// This is useful if you only need the code itself, and not the meta-information.
    /// All Optional fields will be None.
    ///
    /// If you need the code and the meta-information, use `get_codes` instead.
    pub async fn get_codes_slim(&self) -> Result<Vec<Code>, ClientError> {
        let response = self.get("/codes").await?;

        let codes: RetrieveCodesResponse = serde_json::from_str(&response).unwrap();

        Ok(mapping_slim(codes))
    }

    /// Query HTTP PUT `/api/v1/codes` and deserialize the response.
    /// *This requires an API Key.*
    ///
    /// Insert a Code into the remote service.
    #[cfg(feature = "write")]
    pub async fn insert_code(
        &mut self,
        insert_request: write::InsertCodeRequest,
    ) -> Result<Option<i32>, ClientError> {
        let payload = puts::RemoteInsertCodeRequest::from(insert_request);

        let result = self
            .put(
                "/codes",
                &serde_json::to_string(&payload).map_err(ClientError::Serde)?,
            )
            .await?;

        // Should always work, but perhaps the remote service has a different version
        // and now has a changed response?
        match result.parse::<i32>() {
            Ok(id) => Ok(Some(id)),
            Err(_) => Ok(None),
        }
    }

    /// Handles the response from the remote service, checking for errors.
    async fn response(&self, response: reqwest::Response) -> Result<String, ClientError> {
        if !response.status().is_success() {
            let s_err: ErrorResponse =
                serde_json::from_str(&response.text().await.map_err(ClientError::Reqwest)?)
                    .map_err(ClientError::Serde)?;

            return Err(ClientError::ServerError(s_err));
        }

        response.text().await.map_err(ClientError::Reqwest)
    }

    fn user_agent() -> String {
        format!(
            "{}/{} (reqwest; {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS
        )
    }

    pub fn default_client() -> reqwest::Client {
        reqwest::Client::builder()
            .user_agent(Self::user_agent())
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                headers
            })
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    }
}

impl Default for CodesClient {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: None,
            client: Self::default_client(),
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
        let client = CodesClient::default();
        assert!(client.base_url.eq(DEFAULT_BASE_URL));
        assert!(client.api_key.is_none());
    }

    #[test]
    fn test_construct_client_with_api_key() {
        assert!(CodesClient::new(Some(ApiKey::new("foo".to_string())))
            .api_key
            .is_some());
    }

    #[test]
    fn test_construct_client_with_base_url() {
        assert_eq!(
            CodesClient::new_full(None, Some("http://foo.example".to_string()), None).base_url,
            "http://foo.example"
        );
    }

    #[test]
    fn test_client_url() {
        let client = CodesClient::default();

        assert_eq!(client.url("foo"), format!("{}/foo", DEFAULT_BASE_URL));
        assert_eq!(client.url("/foo"), format!("{}/foo", DEFAULT_BASE_URL));
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

    #[test]
    fn test_can_deserialize_rocket_error() {
        let output = serde_json::from_str::<ErrorResponse>(
            r#"{"error":{ "code": 422,"reason":"Unprocessable Entity","description": "data.."}}"#,
        );

        assert!(output.is_ok());
    }

    #[test]
    fn test_can_deserialize_remote_error() {
        let output = serde_json::from_str::<ErrorResponse>(
            r#"{"error":{"code":401,"description":"Invalid API key","debug":null}}"#,
        );

        assert!(output.is_ok());
    }

    #[test]
    #[cfg(feature = "write")]
    fn test_it_can_serialize_insert_request() {
        let insert_request = write::InsertCodeRequest {
            code: "FOOB-BARS-TEST".to_string(),
            expires_at: 800,
            creator: write::SourceLookup {
                name: "Example Creator".to_string(),
                url: "https://creator.example.org".to_string(),
            },
            submitter: Some(write::SourceLookup {
                name: "Example Submitter".to_string(),
                url: "https://submitter.example.org".to_string(),
            }),
        };

        let remote_request = puts::RemoteInsertCodeRequest::from(insert_request);

        let ser = serde_json::to_string(&remote_request);

        assert!(ser.is_ok());
        assert_eq!(
            ser.unwrap(),
            r#"{"code":"FOOB-BARS-TEST","expires_at":800,"creator_name":"Example Creator","creator_url":"https://creator.example.org","submitter_name":"Example Submitter","submitter_url":"https://submitter.example.org"}"#
        );
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
