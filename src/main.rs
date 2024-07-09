use std::env;
use std::process;
use std::fs;
use std::path::Path;
use ssh2::Session;
use std::net::TcpStream;
use std::io::Write;

mod config;
use config::{Config, ConnectionInfo};

fn main() {
    let config = config::load_config();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1].trim() == "help" {
        println!("\nUsage: rust-ssh-connect <ip>\n");
        process::exit(1);
    }

    let server = args[1].trim();
    let combos = generate_combos(server, &config.ssh);

    for combo in combos {
        if try_connect(&combo) {
            process::exit(0);
        }
    }

    println!("Finished (unable to connect)");
    process::exit(1);
}

fn generate_combos(server: &str, ssh_config: &config::SshConfig) -> Vec<ConnectionInfo> {
    let mut combos = Vec::new();
    for cert in &ssh_config.certs {
        for &port in &ssh_config.ports {
            for user in &ssh_config.users {
                combos.push(ConnectionInfo {
                    host: server.to_string(),
                    port,
                    username: user.to_string(),
                    private_key: cert.replace("~", &env::var("HOME").unwrap()),
                });
            }
        }
    }
    combos
}

fn try_connect(info: &ConnectionInfo) -> bool {
    let connection_string = format!("ssh -i {} {}@{} -p{}", 
                                    info.private_key, info.username, info.host, info.port);
    println!("Trying: {}", connection_string);

    match TcpStream::connect(format!("{}:{}", info.host, info.port)) {
        Ok(tcp) => {
            let mut sess = Session::new().unwrap();
            sess.set_tcp_stream(tcp);
            sess.handshake().unwrap();

            match sess.userauth_pubkey_file(&info.username, None, Path::new(&info.private_key), None) {
                Ok(_) => {
                    println!("Connected: {} (copied)", connection_string);
                    copy_to_clipboard(&connection_string);
                    true
                },
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
}

fn copy_to_clipboard(text: &str) {
    if cfg!(target_os = "macos") {
        let mut process = process::Command::new("pbcopy")
            .stdin(process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn pbcopy process");

        if let Some(mut stdin) = process.stdin.take() {
            stdin.write_all(text.as_bytes()).expect("Failed to write to stdin");
        }
    } else {
        println!("Clipboard functionality not implemented for this OS");
    }
}

