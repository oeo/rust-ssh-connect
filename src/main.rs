use clap::Parser;
use log::{info, warn, error};
use std::{process, env};

mod config;
use config::{ConnectionInfo, load_config, find_ssh_keys};

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name")]
struct Opts {
    /// ip address of the server to connect to
    #[clap(name = "server")]
    server: String,

    /// username for ssh connection
    #[clap(short, long)]
    username: Option<String>,

    /// path to the ssh key file
    #[clap(short, long)]
    key_file: Option<String>,

    /// verbose mode
    #[clap(short, long)]
    verbose: bool,

    /// quiet mode
    #[clap(short, long)]
    quiet: bool,

    /// list available ssh keys
    #[clap(long)]
    list_keys: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.list_keys {
        println!("available ssh keys:");
        for key in find_ssh_keys() {
            println!("  {}", key);
        }
        return;
    }

    setup_logger(opts.verbose, opts.quiet);

    let config = load_config();

    let username = opts.username.as_deref().unwrap_or("");
    let key_file = opts.key_file.as_deref().unwrap_or("");

    let mut combos = generate_combos(&opts.server, &config.ssh, username, key_file);

    for combo in &mut combos {
        if try_connect(combo) {
            process::exit(0);
        }
    }

    error!("finished (unable to connect)");
    process::exit(1);
}

fn setup_logger(verbose: bool, quiet: bool) {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", if verbose {
            "debug"
        } else if quiet {
            "error"
        } else {
            "info"
        });

    env_logger::init_from_env(env);
}

fn generate_combos(server: &str, ssh_config: &config::SshConfig, username: &str, key_file: &str) -> Vec<ConnectionInfo> {
    let mut combos = Vec::new();
    let users = if username.is_empty() { ssh_config.users.clone() } else { vec![username.to_string()] };
    let certs = if key_file.is_empty() { ssh_config.certs.clone() } else { vec![key_file.to_string()] };

    for cert in &certs {
        for &port in &ssh_config.ports {
            for user in &users {
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
    info!("trying: {}", connection_string);

    let output = std::process::Command::new("ssh")
        .arg("-i").arg(&info.private_key)
        .arg("-p").arg(info.port.to_string())
        .arg(format!("{}@{}", info.username, info.host))
        .arg("-o").arg("BatchMode=yes")
        .arg("-o").arg("StrictHostKeyChecking=no")
        .arg("-o").arg("NumberOfPasswordPrompts=0")
        .arg("-v")
        .arg("exit")
        .output();

    match output {
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if output.status.success() {
                info!("connected: {} (copied)", connection_string);
                copy_to_clipboard(&connection_string);
                true
            } else {
                warn!("connection failed:");
                if stderr.contains("Permission denied") {
                    warn!("authentication failed. the provided key was not accepted.");
                } else if stderr.contains("Connection refused") {
                    warn!("the server refused the connection. it might be down or not accepting connections on this port.");
                } else if stderr.contains("Connection timed out") {
                    warn!("the connection attempt timed out. the server might be unreachable.");
                } else {
                    warn!("unknown error. full output:");
                    warn!("{}", stderr);
                }
                false
            }
        },
        Err(e) => {
            error!("failed to execute ssh command: {}", e);
            false
        }
    }
}

fn copy_to_clipboard(text: &str) {
    if cfg!(target_os = "macos") {
        let mut process = process::Command::new("pbcopy")
            .stdin(process::Stdio::piped())
            .spawn()
            .expect("failed to spawn pbcopy");

        if let Some(mut stdin) = process.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes()).expect("failed to write to stdin");
        }
    } else {
        println!("clipboard functionality not implemented for this os");
    }
}

