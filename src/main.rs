mod encoder;
mod interface;

#[tokio::main]
async fn main() {
    interface::main().expect("Failed Something");
}
