// External deps
use std::io::Result as ioResult;
use std::ptr;

// Internal deps
use crate::iracing::connection;
use crate::iracing::utils;

pub struct IRacingClient {
    connection: connection::Connection,
    header : *const  utils::IRacingHeader
}

impl IRacingClient {
    pub fn start() -> ioResult<IRacingClient> {
        let con = connection::Connection::establish();
        if !con.is_ok() {
            return Err(con.err().unwrap());
        }
        let con_unwrapped = con.unwrap();
        let temp_header = con_unwrapped.mmf_view as *const utils::IRacingHeader;
        Ok(IRacingClient{connection: con_unwrapped, header: temp_header})
    }

    pub fn end(&mut self) {
        self.header = ptr::null();
        self.connection.close().expect("Failed to close!");
    }
}