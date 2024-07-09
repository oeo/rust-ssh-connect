use clap::Parser;
use log::{info, warn, error};
use std::{process, env};

mod config;
use config::{ConnectionInfo, load_config, find_ssh_keys};

#[derive(Parser)]
#[clap(version = "1.0")]
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
            info!("connected {}@{}", combo.username, combo.host);
            process::exit(0);
        }
    }

    error!("failed to connect");
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
        .arg("-o").arg("ConnectTimeout=3")
        .arg("-v")
        .arg("exit")
        .output();

    match output {
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if output.status.success() {
                true
            } else {
                if stderr.contains("Permission denied") {
                    warn!("authentication failed for {}@{}", info.username, info.host);
                } else if stderr.contains("Connection refused") {
                    warn!("connection refused for {}@{}", info.username, info.host);
                } else if stderr.contains("Connection timed out") {
                    warn!("connection timed out for {}@{}", info.username, info.host);
                } else if stderr.contains("Bad host") {
                    warn!("bad host: {}@{}", info.username, info.host);
                } else {
                    warn!("unknown error connecting to {}@{}", info.username, info.host);
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
