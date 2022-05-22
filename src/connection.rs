use crate::action::basic_qot;
use crate::frame::{Error, Frame};
use bytes::{Buf, BytesMut};
use protobuf::{self, MessageFull};
use std::io::{self, Cursor};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

type BasicQotSender = mpsc::Sender<basic_qot::update::UpdateBasicQotResponse>;
type BasicQotReceiver = Arc<Mutex<mpsc::Receiver<basic_qot::update::UpdateBasicQotResponse>>>;

pub struct Connection {
    stream: BufWriter<TcpStream>,

    buffer: BytesMut,

    sender: BasicQotSender,

    pub receiver: BasicQotReceiver,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        let (tx, rx) = mpsc::channel(10000);

        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub async fn read_frame<T: MessageFull>(
        &mut self,
        proto_id: u32,
    ) -> Result<Option<Frame<T>>, Error> {
        loop {
            let mut buf = Cursor::new(&self.buffer[..]);

            match Frame::parse(&mut buf, proto_id).await {
                Ok((frame, frame2)) => {
                    let len = buf.position() as usize;
                    self.buffer.advance(len);

                    if frame2.is_none() {
                        return Ok(Some(frame.unwrap()));
                    }

                    self.sender.send(frame2.unwrap()).await.unwrap();
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
            .write_all(frame.body.write_to_bytes().unwrap().as_ref())
            .await?;
        self.stream.flush().await
    }
}
