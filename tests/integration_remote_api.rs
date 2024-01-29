use licc::client::error::ClientError;
use licc::client::CodesClient;
use licc::Code;

// Connects to a remote API and retrieves codes.
#[tokio::test]
async fn test_get_codes() {
    let client = CodesClient::default();

    let response: Vec<Code> = client.get_codes().await.unwrap();

    assert!(!response.is_empty());

    let first: &Code = response.first().unwrap();

    assert!(first.code.len() > 12);
}

#[tokio::test]
async fn test_404() {
    let client = CodesClient::default();

    let response = client.get("/inttests_client/some_nonexistent").await;

    assert!(response.is_err());

    let err = response.unwrap_err();

    if let ClientError::ServerError(err) = err {
        assert_eq!(err.error.code, 404);
    } else {
        unreachable!("Expected ServerError, got {:?}", err);
    }
}
