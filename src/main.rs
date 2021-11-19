use log::error;
use std::env;

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }

    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];

    match protocol {
        "tcp" => match role {
            "server" => todo!(),
            "client" => todo!(),
            _ => todo!(),
        },
        "udp" => match role {
            "server" => todo!(),
            "client" => todo!(),
            _ => todo!(),
        },
        _ => {
            todo!()
        }
    }
}
