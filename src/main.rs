mod client;
mod config;
mod srun;
mod tool;
use core::time;
use std::fs::File;
use std::thread::sleep;

use daemonize::Daemonize;
use simple_logger::SimpleLogger;

fn run(config_path: &str) {
    let run_config = config::Config::load(config_path);
    if run_config.accounts.len() == 0 {
        log::error!("no account found");
        return;
    }
    if run_config.daemon{
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
                    sleep(time::Duration::from_secs(5));
                    println!("Check")
                }
            }
            Err(e) => eprintln!("Error, {}", e),
        }
    }else{
        println!("{:#}", serde_yaml::to_string(&run_config).unwrap());
        let mut client = client::Client::new();
        client.add_auth(run_config);
        client.login_all();
    }
}
fn main() {
    SimpleLogger::new().init().unwrap();
    let config_path = "./config.yaml";
    run(config_path);
}
