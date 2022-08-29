use fireblocks_sdk::CallbackType;
use tokio::fs;


#[tokio::main]
async fn main() {
    let file = fs::read_to_string("./test.json").await.unwrap();
    let result = CallbackType::serialize(&file).await;

    println!("{}


    println!("{}", file);

}
