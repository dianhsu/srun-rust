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
        let (_, status) = self.info.login();
        log::info!("{} login status: {}", self.info.username, status);
        return status;
    }
    fn logout(&mut self) -> bool {
        let status = self.info.logout();
        log::info!("{} logout status: {}", self.info.username, status);
        return status;
    }
    fn status(&mut self) -> bool {
        let status = self.info.status();
        log::info!("{} status: {}", self.info.username, status);
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
    pub fn logout_all(&mut self) {
        for auth in self.auths.iter_mut() {
            let logout_status = auth.logout();
            match logout_status {
                true => {
                    auth.status = false;
                }
                false => {
                    auth.status = true;
                }
            }
        }
    }
    pub fn status_all(&mut self) {
        for auth in self.auths.iter_mut() {
            auth.status = auth.status();
        }
    }
}
