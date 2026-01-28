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

    loop {
        sleep(Duration::from_secs(5));
        let Ok(mut stream) = TcpStream::connect((hostname.as_str(), port)) else {
            continue;
        };
        let Ok(response) = ping(&mut stream, &hostname, port) else {
            continue;
        };
        ("playing: {}", response.online_players);
    }
}
