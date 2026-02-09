use craftping::sync::ping;

use std::env::args;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let hostname = args()
        .skip(1)
        .next()
        .expect("Needs host (dns or ip) as first argument");
    let port = args()
        .skip(2)
        .next()
        .expect("Needs port as second argument")
        .parse()
        .expect("Port should be a number");

    let mut first = true;
    loop {
        sleep(Duration::from_secs(5));

        let mut stream = match TcpStream::connect((hostname.as_str(), port)) {
            Ok(stream) => stream,
            Err(e) => {
                if first {
                    eprintln!("error connecting: {e}");
                };
                first = false;
                continue;
            }
        };
        let response = match ping(&mut stream, &hostname, port) {
            Ok(response) => response,
            Err(e) => {
                if first {
                    eprintln!("error pinging: {e}");
                };
                first = false;
                continue;
            }
        };
        println!("playing: {}", response.online_players);
    }
}
