use licc::client::CodesClient;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = CodesClient::default();

    // Use _slim() because we only want to print the `.code` field, not the entire struct.
    // If we need more meta-information, we can use the regular get_codes() function.
    let result = client.get_codes_slim().await;

    match result {
        Ok(codes) => {
            for code in codes {
                println!("Code: {}", code.code);
            }
        }
        Err(err) => println!("Error listing code: {:?}", err),
    };
}
