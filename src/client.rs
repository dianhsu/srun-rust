use crate::{config::Config, srun::LoginInfo};

pub struct Auth {
    info: LoginInfo,
    status: bool,
}
pub struct Client {
    auths: Vec<Auth>,
}
impl Auth {
    fn new(info: LoginInfo) -> Auth {
        Auth {
            info: info,
            status: false,
        }
    }
    fn login(&mut self) -> bool {
        let (info, status) = self.info.login();
        log::info!("{} login status: {}, \'{}\'", self.info.username, status, info);
        return status;
    }
}
impl Client {
    pub fn new() -> Client {
        Client { auths: Vec::new() }
    }
    pub fn add_auth(&mut self, config: Config) {
        for account in config.accounts.iter() {
            let info = LoginInfo::new(
                &account.username,
                &account.password,
                &config.basic.host,
                &config.basic.ac_id,
                &config.basic.enc,
                &config.basic.n,
                &config.basic.vtype,
                config.basic.https,
                &account.interface,
            );
            self.auths.push(Auth::new(info));
        }
    }
    pub fn login_all(&mut self) {
        for auth in self.auths.iter_mut() {
            auth.status = auth.login();
        }
    }
}
