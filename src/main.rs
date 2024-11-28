mod uit;
#[tokio::main]
async fn main() {
    tokio::join!(uit::start());
}