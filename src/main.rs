mod encoder;

#[tokio::main]
async fn main() {
    encoder::process_file("input.mp4", "output.mp4").await;
}
