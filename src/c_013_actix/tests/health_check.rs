use c_013_actix::run;
use reqwest::Client;

async fn spawn_app() -> std::io::Result<()>{
    run().await
}

#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8888/health_check")
        .send()
        .await
        .expect("Request failed");

    assert!(response.status().is_success());

}