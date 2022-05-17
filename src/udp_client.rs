use std::net::UdpSocket;
use std::{io, str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send_to(input.as_bytes(), address)?;

        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!(
            "{}",
            str::from_utf8(&buffer).expect("failed to convert to String")
        );
    }

    // thread 'main' panicked at 'Failed to send data: Os { code: 40, kind: Uncategorized, message: "Message too long" }', src/udp_client.rs:21:10
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // let input = "a".to_string().repeat(70000);

    // let input = "a".to_string().repeat(65507);
    // let input = "a".to_string().repeat(9216);

    // socket
    //     .send_to(input.as_bytes(), &address)
    //     .expect("Failed to send data");

    Ok(())
}
