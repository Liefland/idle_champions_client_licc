use licc::{
    api_key::ApiKey,
    client::CodesClient,
    write::{InsertCodeRequest, SourceLookup},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api_key = ApiKey::new("example".to_string());
    let mut client = CodesClient::new(Some(api_key));

    let current_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();

    let result = client
        .insert_code(InsertCodeRequest {
            code: "FOOS-BARS-CODE".to_string(),
            expires_at: current_epoch.as_secs() + 604800,
            creator: SourceLookup {
                name: "Example Creator".to_string(),
                url: "https://creator.example.org".to_string(),
            },
            submitter: Some(SourceLookup {
                name: "Example Submitter".to_string(),
                url: "https://submitter.example.org".to_string(),
            }),
        })
        .await;

    match result {
        Ok(id) => println!("Code inserted successfully! It has ID: {:?}", id),
        Err(err) => println!("Error inserting code: {:?}", err),
    };
}
