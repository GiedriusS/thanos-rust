use hyper::Client;

#[derive(Default)]
pub struct PrometheusClient {
    pub url: String,
}

impl PrometheusClient {
    pub async fn get_status(&self) {
        println!("hello!");

        // Still inside `async fn main`...
        let client = Client::new();

        // Parse an `http::Uri`...
        let uri = self.url.parse().unwrap();

        // Await the response...
        let mut resp = client.get(uri).await.unwrap();

        println!("Response: {}", resp.status());
    }
}
