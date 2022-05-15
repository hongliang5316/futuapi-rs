use crate::frame::{Error, Frame};
use bytes::{Buf, BytesMut};
use protobuf::MessageFull;
use std::io::{self, Cursor};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

pub struct Connection {
    stream: BufWriter<TcpStream>,

    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_frame<T: MessageFull>(&mut self) -> Result<Option<Frame<T>>, Error> {
        loop {
            let mut buf = Cursor::new(&self.buffer[..]);

            match Frame::parse(&mut buf) {
                Ok(frame) => {
                    let len = buf.position() as usize;
                    self.buffer.advance(len);
                    return Ok(Some(frame));
                }
                Err(Error::Incomplete) => {
                    if 0 == self.stream.read_buf(&mut self.buffer).await.unwrap() {
                        if self.buffer.is_empty() {
                            return Ok(None);
                        } else {
                            return Err(Error::Other("connection reset by peer".into()));
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    pub async fn write_frame<T: MessageFull>(&mut self, frame: &Frame<T>) -> io::Result<()> {
        self.stream.write_all(&frame.header.to_vec()).await?;
        self.stream
            .write_all(
                protobuf_json_mapping::print_to_string(&frame.body)
                    .unwrap()
                    .as_bytes(),
            )
            .await?;
        self.stream.flush().await
    }
}
