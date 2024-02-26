use std::{time::Duration, net::SocketAddr};
use tokio::{time::Instant, io::{AsyncWriteExt, AsyncReadExt}};
use tokio::net::{TcpListener, TcpStream};

const HB_REQUEST: u8 = 1;
const HB_REPLY: u8 = 0;

#[tokio::main]
pub async fn main() {
    // Pass process ID via cmd args
    let pid: u64 = std::env::args().skip(1).next().unwrap().parse().unwrap();

    // Process 1 waits for an incoming connection and process 2 connects to 1
    let listening_address = SocketAddr::from(([127, 0, 0, 1], 8001));
    let (mut reader, mut writer) = if pid == 1 {
        let listener = TcpListener::bind(listening_address).await.unwrap();
        let (conn, _addr) = listener.accept().await.unwrap();
        conn.into_split()
    } else if pid == 2 {
        TcpStream::connect(listening_address).await.unwrap().into_split()
    } else {
        unimplemented!()
    };

    // Processes send/reply to heartbeats and measure response time
    // let mut interval = tokio::time::interval(Duration::from_millis(100));
    let mut interval = tokio::time::interval(Duration::from_millis(100 + (87 * pid)));
    let mut heartbeat_start = Instant::now();
    loop {
        tokio::select! {
            _ = interval.tick() => {
                heartbeat_start = Instant::now();
                writer.write_u8(HB_REQUEST).await.unwrap();
                writer.flush().await.unwrap();
            },
            msg = reader.read_u8() => {
                match msg.unwrap() {
                    HB_REQUEST => {
                        writer.write_u8(HB_REPLY).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    HB_REPLY => {
                        let message_delay = (Instant::now() - heartbeat_start).as_millis();
                        println!("{message_delay}");
                    }
                    _ => unimplemented!(),
                }
            },
        }
    };
}

