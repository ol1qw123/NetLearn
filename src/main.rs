use uit::start;

mod uit;
#[tokio::main]
async fn main() {
    
    if let (Err(e) ,) = tokio::join!(start()){
        eprintln!("Error in start function: {}", e);
    };


}
