use bytes::BufMut;
use bytes::BytesMut;
use std::iter::Iterator;
use std::{io, str};
use tokio_serial::{Error, Serial, SerialPortSettings};
use tokio_util::codec::Framed;
use tokio_util::codec::{Decoder, Encoder};

pub struct Codec;

impl Decoder for Codec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder<String> for Codec {
    type Error = io::Error;

    fn encode(&mut self, line: String, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.reserve(line.len() + 1);
        buf.put_slice(line.as_bytes());
        buf.put_u8(b'\n');
        Ok(())
    }
}

pub type SerialTransport = Framed<Serial, Codec>;

pub fn serial(tty_path: &str, baud_rate: u32) -> Result<SerialTransport, Error> {
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = baud_rate;
    let mut port = Serial::from_path(tty_path, &settings)?;
    #[cfg(unix)]
    port.set_exclusive(false)?;

    Ok(Codec.framed(port))
}
