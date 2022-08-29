use tokio::fs;


#[tokio::main]
async fn main() {
    let file = fs::read_to_string("./test.json").await.unwrap();
    CallbackType::


    println!("{}", file);

}
