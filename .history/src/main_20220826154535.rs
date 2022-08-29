use tokio::fs;


#[tokio::main]
async fn main() {
    let file = fs::read_to_string("src/main.rs").await.unwrap();
}
