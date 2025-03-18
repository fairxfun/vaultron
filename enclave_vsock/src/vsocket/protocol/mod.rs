mod handler;
mod header;

pub use handler::*;
pub use header::*;

use crate::VsockProtocolError;
use std::io::{Read, Write};
use vsock::VsockStream;

pub trait ProtocolStreamTrait {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), VsockProtocolError>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), VsockProtocolError>;
    fn flush(&mut self) -> Result<(), VsockProtocolError>;
}

impl ProtocolStreamTrait for VsockStream {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), VsockProtocolError> {
        Read::read_exact(self, buf).map_err(VsockProtocolError::IoError)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), VsockProtocolError> {
        Write::write_all(self, buf).map_err(VsockProtocolError::IoError)
    }

    fn flush(&mut self) -> Result<(), VsockProtocolError> {
        Write::flush(self).map_err(VsockProtocolError::IoError)
    }
}
