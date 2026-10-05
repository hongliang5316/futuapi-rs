use crate::frame::{Error, Frame, FrameRaw};
use bytes::{Buf, BytesMut};
use protobuf::MessageFull;
use std::io::{self, Cursor};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

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
                    if 0 == self
                        .stream
                        .read_buf(&mut self.buffer)
                        .await
                        .map_err(|e| Error::ConnectionError(e.to_string()))?
                    {
                        if self.buffer.is_empty() {
                            return Ok(None);
                        } else {
                            return Err(Error::ConnectionError("connection reset by peer".into()));
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
        self.stream.write_all(&frame.to_bytes()?).await?;
        self.stream.flush().await
    }
}

/// Reads raw frames from the read half of a connection, leaving the proto type
/// to be decided by the caller from `header.proto_id`.
pub struct FrameReader<R> {
    stream: R,

    buffer: BytesMut,
}

impl<R: AsyncRead + Unpin> FrameReader<R> {
    pub fn new(stream: R) -> Self {
        FrameReader {
            stream,
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_frame_raw(&mut self) -> Result<Option<FrameRaw>, Error> {
        loop {
            let mut buf = Cursor::new(&self.buffer[..]);

            match FrameRaw::parse(&mut buf) {
                Ok(frame) => {
                    let len = buf.position() as usize;
                    self.buffer.advance(len);
                    return Ok(Some(frame));
                }
                Err(Error::Incomplete) => {
                    if 0 == self
                        .stream
                        .read_buf(&mut self.buffer)
                        .await
                        .map_err(|e| Error::ConnectionError(e.to_string()))?
                    {
                        if self.buffer.is_empty() {
                            // maybe server gracefully close the connection
                            return Ok(None);
                        } else {
                            return Err(Error::ConnectionError("connection reset by peer".into()));
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
