mod base64;
mod md5;
mod sha1;
mod xencode;
use self::base64::get_base64;
use self::md5::get_md5;
use self::sha1::get_sha1;
use crate::tool::{get_ip_from_interface, rand_str};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use urlencoding::encode;
use xencode::get_xencode;
#[derive(Debug, Clone)]
pub struct LoginInfo {
    pub callback: String,
    pub username: String,
    pub password: String,
    pub ac_id: String,
    pub enc: String,
    pub info: String,
    pub ip: String,
    pub chksum: String,
    pub n: String,
    pub vtype: String,
    pub interface: String,
    pub host: String,
    pub login_page: String,
    pub status_url: String,
    pub request_url: String,
    pub challenge_url: String,
}
#[derive(Serialize, Deserialize)]
struct GenerateInfo {
    username: String,
    password: String,
    ip: String,
    acid: String,
    enc_ver: String,
}

use curl::easy::{Easy2, Handler, List, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn request(url: &str, interface: &str) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    if !interface.is_empty() {
        easy.interface(interface).expect("");
    }
    easy.get(true).unwrap();
    easy.url(url).unwrap();
    let mut list = List::new();
    list.append("User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.26 Safari/537.36").unwrap();
    easy.http_headers(list).unwrap();
    match easy.perform() {
        Ok(_) => {}
        Err(e) => {
            log::error!("request error: {}", e);
            return String::from("");
        }
    }
    let contents = easy.get_ref();
    let res = String::from_utf8_lossy(&contents.0).to_string();
    log::debug!("response: {}", res);
    return res;
}
impl LoginInfo {
    pub fn new(
        username: &str,
        password: &str,
        host: &str,
        ac_id: &str,
        enc: &str,
        n: &str,
        vtype: &str,
        https: bool,
        interface: &str,
    ) -> Self {
        let callback = "jsonp1583251661368_".to_string() + &rand_str();
        let info = "";
        let chksum = "";
        LoginInfo {
            callback: callback.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            ac_id: ac_id.to_string(),
            enc: enc.to_string(),
            info: info.to_string(),
            ip: "".to_string(),
            chksum: chksum.to_string(),
            n: n.to_string(),
            vtype: vtype.to_string(),
            interface: interface.to_string(),
            host: host.to_string(),
            login_page: format!(
                "{}://{}/cgi-bin/srun_portal_pc",
                (if https { "https" } else { "http" }),
                host
            ),
            status_url: format!(
                "{}://{}/cgi-bin/rad_user_info",
                (if https { "https" } else { "http" }),
                host
            ),
            request_url: format!(
                "{}://{}/cgi-bin/srun_portal",
                (if https { "https" } else { "http" }),
                host
            ),
            challenge_url: format!(
                "{}://{}/cgi-bin/get_challenge",
                (if https { "https" } else { "http" }),
                host
            ),
        }
    }
    pub fn _send_login_info(&self) -> String {
        let url = format!("{}?callback={}&action=login&username={}&password={}&ac_id={}&ip={}&info={}&chksum={}&n={}&type={}&os=Windows+10&name=windows&double_stack=0",
        self.request_url, self.callback, self.username, encode(&self.password).to_string(), self.ac_id, self.ip, encode(&self.info).to_string(),encode(&self.chksum).to_string(),self.n,self.vtype);
        log::debug!("login url: {:?}", url);
        request(&url, &self.interface)
    }
    pub fn login(&mut self) -> (String, bool) {
        self.ip = get_ip_from_interface(&self.interface);
        let token = self.get_token();
        self._get_login_response(&token)
    }
    fn _get_login_response(&mut self, token: &str) -> (String, bool) {
        self.chksum = self._generate_encrypted_login_info(token);
        let login_response = self._send_login_info();
        let login_result = Self::_resolve_login_response(login_response.as_str());
        return login_result;
    }
    fn _resolve_login_response(page_text: &str) -> (String, bool) {
        log::debug!("login response: {}", page_text);
        let re = Regex::new("\"suc_msg\":\"(.*?)\"").unwrap();
        for cap in re.captures_iter(page_text) {
            return (cap[1].to_string(), true);
        }
        return ("".to_string(), false);
    }
    fn _get_login_page(self) -> String {
        return request(&self.login_page, &self.interface);
    }
    fn get_token(&self) -> String {
        let challenge_response = self._get_challenge();
        let token = Self::_resolve_token_from_challenge_response(&challenge_response);
        log::debug!("token: {}", token);
        return token;
    }
    fn _get_challenge(&self) -> String {
        let url = format!(
            "{}?callback={}&username={}&ip={}",
            self.challenge_url, self.callback, self.username, self.ip
        );
        log::debug!("challenge url: {}", url);

        return request(url.as_str(), &self.interface);
    }
    fn _resolve_token_from_challenge_response(page_text: &str) -> String {
        let re = Regex::new("\"challenge\":\"(.*?)\"").unwrap();
        for cap in re.captures_iter(page_text) {
            return cap[1].to_string();
        }
        return "".to_string();
    }
    fn _generate_encrypted_login_info(&mut self, token: &str) -> String {
        let info = self.clone()._generate_info();
        self.info = Self::_encrypt_info(&info, token);
        log::debug!("encrypt info: {}", self.info);
        let md5 = self._generate_md5(token);
        self.password = Self::_encrypt_md5(md5.as_str());
        let chksum = self.clone()._generate_chksum(token, &md5, &self.info);
        self._encrypt_chksum(&chksum)
    }
    fn _generate_info(&self) -> String {
        let info = GenerateInfo {
            username: self.username.clone(),
            password: self.password.clone(),
            ip: self.ip.clone(),
            acid: self.ac_id.clone(),
            enc_ver: self.enc.clone(),
        };
        let info = serde_json::to_string(&info).unwrap();
        log::debug!("info: {}", info);
        return info;
    }
    fn _encrypt_info(info: &str, token: &str) -> String {
        let xen = get_xencode(info, token);
        return String::from("{SRBX1}") + &get_base64(&xen);
    }
    fn _generate_md5(&self, token: &str) -> String {
        return get_md5(&self.password, token);
    }
    fn _encrypt_md5(md5: &str) -> String {
        return format!("{{MD5}}{}", md5);
    }
    fn _generate_chksum(&self, token: &str, enc_md5: &str, enc_info: &str) -> String {
        return format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            token,
            self.username,
            token,
            enc_md5,
            token,
            self.ac_id,
            token,
            self.ip,
            token,
            self.n,
            token,
            self.vtype,
            token,
            enc_info
        );
    }
    fn _encrypt_chksum(&self, chksum: &str) -> String {
        get_sha1(chksum)
    }
    pub fn logout(&self) -> bool {
        let logout_response = self._get_logout_response();
        log::debug!("logout response: {}", logout_response);
        let (logout_result, res) = self._resolve_logout_response(logout_response.as_str());
        log::debug!("logout result: {}", logout_result);
        return res;
    }
    pub fn _get_logout_response(&self) -> String {
        let url = format!(
            "{}?callback={}&username={}&ip={}&action=logout&ac_id={}",
            self.request_url, self.callback, self.username, self.ip, self.ac_id
        );
        request(&url, &self.interface)
    }
    pub fn _resolve_logout_response(&self, page_text: &str) -> (String, bool) {
        let re = Regex::new("\"error_msg\":\"(.*?)\"").unwrap();
        for cap in re.captures_iter(page_text) {
            return (cap[1].to_string(), true);
        }
        return ("".to_string(), false);
    }
    pub fn status(&self) -> bool {
        let status_response = self._get_status_response();
        log::debug!("status response: {}", status_response);
        return self._resolve_status_reponse(status_response.as_str());
    }
    fn _get_status_response(&self) -> String {
        let url = format!("{}?callback={}", self.status_url, self.callback);
        request(&url, &self.interface)
    }
    fn _resolve_status_reponse(&self, page_text: &str) -> bool {
        let re = Regex::new("\"user_name\":\"(.*?)\"").unwrap();
        for _ in re.captures_iter(page_text) {
            return true;
        }
        return false;
    }
}
