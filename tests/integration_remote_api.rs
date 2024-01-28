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
