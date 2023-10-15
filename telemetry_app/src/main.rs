use telemetry_reader_lib::iracing;

fn main() {
    let client_result = iracing::iracing_client::IRacingClient::start();
    if !client_result.is_ok() {
        print!("Failed to establish connection to IRacing memory mapped file.");
    }

    let mut client = client_result.unwrap();
    client.end();
}
