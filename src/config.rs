use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process;
#[derive(Deserialize, Serialize)]
pub struct Account {
    pub interface: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Basic {
    pub host: String,
    pub ac_id: String,
    pub enc: String,
    pub n: String,
    pub vtype: String,
    pub https: bool,
    pub timeout: u64,
}
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub basic: Basic,
    pub accounts: Vec<Account>,
    pub daemon: bool,
    pub log_path: String,
}

impl Config {
    pub fn load(config_path: &str) -> Config {
        let path = Path::new(config_path);
        if !path.exists() {
            let mut accounts: Vec<Account> = Vec::new();
            accounts.push(Account {
                interface: "".to_string(),
                username: "".to_string(),
                password: "".to_string(),
            });
            let config = Config {
                basic: Basic {
                    host: "127.0.0.1".to_string(),
                    ac_id: "7".to_string(),
                    enc: "srun_bx1".to_string(),
                    n: "200".to_string(),
                    vtype: "1".to_string(),
                    https: false,
                    timeout: 300,
                },
                accounts: accounts,
                daemon: false,
                log_path: "/var/log/srun.log".to_string()
            };
            let display = path.display();
            let write_config = serde_yaml::to_string(&config).unwrap();
            match File::create(&path) {
                Ok(mut file) => match file.write_all(write_config.as_bytes()) {
                    Err(why) => {
                        log::error!("couldn't write to {}: {:?}", display, why);
                        process::exit(100);
                    }
                    Ok(_) => {
                        log::info!("successfully wrote to {}", display);
                        process::exit(0);
                    }
                },
                Err(why) => {
                    log::error!("couldn't write to {}: {:?}", display, why);
                    process::exit(100);
                }
            };
        } else {
            let mut file = match File::open(&path) {
                Err(_) => {
                    process::exit(101);
                }
                Ok(file) => file,
            };
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => {
                    log::error!("couldn't read from {}: {:?}", path.display(), why);
                    process::exit(102);
                }
                Ok(_) => {
                    log::debug!("successfully read from {}", path.display());
                }
            }
            serde_yaml::from_str(s.as_str()).unwrap()
        }
    }
}
