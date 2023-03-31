mod client;
mod config;
mod srun;
mod tool;
use core::time;
use std::fs::File;
use std::thread::sleep;

use daemonize::Daemonize;

fn run(config_path: &str, action: &str) {
    let run_config = config::Config::load(config_path);
    if run_config.accounts.len() == 0 {
        log::error!("no account found");
        return;
    }
    match action {
        "login" => {
            if run_config.daemon {
                log::info!("daemon mode");
                let stdout = File::create("/tmp/srun.out").unwrap();
                let stderr = File::create("/tmp/srun.err").unwrap();
                let daemonize = Daemonize::new()
                    .pid_file("/var/run/srun/srun.pid") // Every method except `new` and `start`
                    .chown_pid_file(true)
                    .working_directory("/tmp") // for default behaviour.
                    .stdout(stdout)
                    .stderr(stderr)
                    .privileged_action(|| "Executed before drop privileges");

                match daemonize.start() {
                    Ok(_) => {
                        println!("Success, daemonized");
                        loop {
                            sleep(time::Duration::from_secs(run_config.basic.timeout));
                        }
                    }
                    Err(e) => log::error!("Error, {}", e),
                }
            } else {
                let mut client = client::Client::new();
                client.add_auth(run_config);
                client.login_all();
            }
        }
        "logout" => {
            let mut client = client::Client::new();
            client.add_auth(run_config);
            client.logout_all();
        }
        "status" => {
            let mut client = client::Client::new();
            client.add_auth(run_config);
            client.status_all();
        }
        _ => {
            log::error!("unknown action");
        }
    }
}

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Action to perform. (login, logout, status) [default=login]
    #[arg(short, long, default_value = "login")]
    action: Option<String>,
    // Path to config file. [default=./config.yaml]
    #[arg(short, long, default_value = "./config.yaml")]
    config_path: String,
    // Log level. (error, warn, info, debug, trace) [default=info]
    #[arg(short, long, default_value = "info")]
    log_level: Option<log::Level>,
}
fn main() {
    let args = Args::parse();
    simple_logger::init_with_level(args.log_level.unwrap()).unwrap();
    run(&args.config_path, &args.action.unwrap());
}
