use hyper::Client;

#[actix_rt::test]
async fn get_backend() {
    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse().unwrap();

    let resp = client.get(uri).await.unwrap();

    println!("Response: {}", resp.status());
}
