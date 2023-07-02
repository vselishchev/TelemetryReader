use telemetry_reader_lib::iracing::connection;

fn main() {
    let con = connection::Connection::establish();
    if con.is_ok() {
        con.unwrap().close().expect("failed to close;");
    }
}
