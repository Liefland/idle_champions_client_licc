use licc::client::CodesClient;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = CodesClient::new_full(None, Some("http://localhost:8000/v1".to_string()), None);

    let result = client.get_codes_slim().await;

    match result {
        Ok(codes) => {
            println!("{:?}", codes.first().unwrap());
        }
        Err(err) => println!("Error fetching codes: {:?}", err),
    };
}
