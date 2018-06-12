// Copyright 2015-2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::net::SocketAddr;

/// A DNS message in serialized form, with either the target address or source address
pub struct SerialMessage {
    message: Vec<u8>,
    addr: SocketAddr,
}

impl SerialMessage {
    /// Construct a new SerialMessage and the source or destination address
    pub fn new(message: Vec<u8>, addr: SocketAddr) -> Self {
        SerialMessage { message, addr }
    }

    /// Get a reference to the bytes
    pub fn bytes(&self) -> &[u8] {
        &self.message
    }

    /// Get the source or destination address (context dependent)
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Unwrap the the Bytes and address
    pub fn unwrap(self) -> (Vec<u8>, SocketAddr) {
        (self.message, self.addr)
    }
}
