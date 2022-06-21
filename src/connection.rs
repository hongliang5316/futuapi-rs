#![allow(unused_mut)]

use crate::action::basic_qot::update::UpdateData;
use crate::frame::{Error, Frame};
use bytes::{Buf, BytesMut};
use protobuf::MessageFull;
use std::io::{self, Cursor};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;
use tokio::time::timeout;

pub struct Connection {
    stream: BufWriter<TcpStream>,

    buffer: BytesMut,

    update_data_channel: UpdateDataChannel,
}

pub struct UpdateDataChannel(mpsc::Sender<UpdateData>, mpsc::Receiver<UpdateData>);

impl UpdateDataChannel {
    fn new(tx: mpsc::Sender<UpdateData>, mut rx: mpsc::Receiver<UpdateData>) -> Self {
        UpdateDataChannel(tx, rx)
    }
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        let (tx, mut rx) = mpsc::channel(1);

        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
            update_data_channel: UpdateDataChannel::new(tx, rx),
        }
    }

    pub fn send_data(&mut self, data: UpdateData) {
        self.update_data_channel.0.try_send(data).unwrap();
    }

    pub async fn read_frame<T: MessageFull>(
        &mut self,
        proto_id: u32,
    ) -> Result<Option<Frame<T>>, Error> {
        loop {
            let mut buf = Cursor::new(&self.buffer[..]);

            match Frame::parse(&mut buf, proto_id) {
                Ok(frame) => {
                    let len = buf.position() as usize;
                    self.buffer.advance(len);

                    if let Some(update_data) = frame.other {
                        if let Err(e) = self.update_data_channel.0.try_send(update_data) {
                            match e {
                                TrySendError::Full(_) => {
                                    println!("full full full full full full");
                                    continue;
                                }
                                TrySendError::Closed(_) => {
                                    return Err(Error::Other(
                                        "The UpdateData channel closed".into(),
                                    ));
                                }
                            }
                        }

                        continue;
                    }

                    return Ok(Some(frame));
                }
                Err(Error::Incomplete) => {
                    match timeout(
                        Duration::from_millis(100),
                        self.stream.read_buf(&mut self.buffer),
                    )
                    .await
                    {
                        Ok(x) => {
                            println!("{:?}", x);
                        }
                        Err(_) => {
                            println!("error error");
                        }
                    };

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
            .write_all(frame.body.write_to_bytes().unwrap().as_ref())
            .await?;
        self.stream.flush().await
    }
}
