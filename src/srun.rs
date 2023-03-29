use regex::Regex;
#[derive(Debug, Clone)]
pub struct LoginInfo {
    pub callback: String,
    pub action: String,
    pub username: String,
    pub password: String,
    pub ac_id: String,
    pub enc: String,
    pub info: String,
    pub chksum: String,
    pub n: String,
    pub vtype: String,
    pub interface: String,
    pub host: String,
    pub https: bool,
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
    easy.perform().unwrap();
    let contents = easy.get_ref();
    return String::from_utf8_lossy(&contents.0).to_string();
}
impl LoginInfo {
    pub fn _send_login_info(self, ip: &str) -> String {
        let url = format!("{}://{}/cgi-bin/srun_protal?callback={}&action={}&username={}&password={}&ac_id={}&ip={}&info={}&chksum={}&n={}&type={}&os=Windows 10&name=windows&double_stack=0",
        (if self.https {"https"} else {"http"}),
        self.host, self.callback, self.action, self.username, self.password, self.ac_id, ip,self.info,self.chksum,self.n,self.vtype);
        request(url.as_str(), "")
    }
    pub fn login(self) -> String {
        let ip = self.clone().get_ip();
        let token = self.clone().get_token(ip.as_str());
        let login_response = self.get_login_response(token.as_str(), ip.as_str());
        return login_response;
    }
    pub fn get_login_response(self, token: &str, ip: &str) -> String {
        self.clone()._generate_encrypted_login_info(token, ip);
        let login_response = self._send_login_info(ip);
        let login_result = Self::_resolve_login_response(login_response.as_str());
        return login_result;
    }
    fn _resolve_login_response(page_text: &str) -> String {
        let re = Regex::new("\"suc_msg\":\"(.*?)\"").unwrap();
        for cap in re.captures_iter(page_text) {
            return cap[1].to_string();
        }
        return "".to_string();
    }
    fn get_ip(self) -> String {
        let page_text = self._get_login_page();
        let ip = Self::_resolve_ip_from_login_page(page_text.as_str());
        return ip;
    }
    fn _get_login_page(self) -> String {
        let url_login_page = format!(
            "{}://{}/srun_portal_pc?ac_id=8&theme=bit",
            (if self.https { "https" } else { "http" }),
            self.host
        );
        return request(url_login_page.as_str(), &self.interface);
    }
    fn _resolve_ip_from_login_page(page_text: &str) -> String {
        let re = Regex::new("id=\"user_ip\" value=\"(.*?)\"").unwrap();
        for cap in re.captures_iter(page_text) {
            return cap[1].to_string();
        }
        return "".to_string();
    }
    fn get_token(self, ip: &str) -> String {
        let challenge_response = self._get_challenge(ip);
        let token = Self::_resolve_token_from_challenge_response(&challenge_response);
        return token;
    }
    fn _get_challenge(self, ip: &str) -> String {
        let url = format!(
            "{}://{}/cgi-bin/srun_protal?callback={}&username={}&ip={}",
            (if self.https { "https" } else { "http" }),
            self.host,
            self.callback,
            self.username,
            ip
        );
        return request(url.as_str(), &self.interface);
    }
    fn _resolve_token_from_challenge_response(page_text: &str) -> String {
        let re = Regex::new("\"challenge\":\"(.*?)\"").unwrap();
        for cap in re.captures_iter(page_text) {
            return cap[1].to_string();
        }
        return "".to_string();
    }
    fn _generate_encrypted_login_info(self, token: &str, ip: &str) -> String {
        let info = self.clone()._generate_info(ip);
        let enc_info = Self::_encrypt_info(info.as_str(), token);
        let md5 = Self::_generate_md5(token);
        let enc_md5 = Self::_encrypt_md5(md5.as_str());
        let chksum = self
            .clone()
            ._generate_chksum(token, enc_md5.as_str(), enc_info.as_str(), ip);
        let enc_chksum = Self::_encrypt_chksum(chksum.as_str());
        return enc_chksum;
    }
    fn _generate_info(self, ip: &str) -> String {
        let info = format!("{{\"username\":\"{}\",\"password\":\"{}\",\"ip\":\"{}\",\"acid\":\"{}\",\"enc_ver\":\"{}\"}}", self.username, self.password, ip, self.ac_id, self.enc);
        return info;
    }
    fn _encrypt_info(info: &str, token: &str) -> String {
        return format!("{{SRBX1}}{}", get_base64(get_xencode(info, token).as_str()));
    }
    fn _generate_md5(token: &str) -> String {
        return get_md5("", token);
    }
    fn _encrypt_md5(md5: &str) -> String {
        return format!("{{MD5}}{}", md5);
    }
    fn _generate_chksum(self, token: &str, enc_md5: &str, enc_info: &str, ip: &str) -> String {
        return format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            token,
            self.username,
            token,
            enc_md5,
            token,
            self.ac_id,
            token,
            ip,
            token,
            self.n,
            token,
            self.vtype,
            token,
            enc_info
        );
    }
    fn _encrypt_chksum(chksum: &str) -> String {
        return get_sha1(chksum);
    }
}

pub fn get_md5(_password: &str, _token: &str) -> String {
    unimplemented!()
}

pub fn get_sha1(_value: &str) -> String {
    unimplemented!()
}
pub fn get_base64(_s: &str) -> String {
    unimplemented!()
}
pub fn get_xencode(_msg: &str, _key: &str) -> String {
    unimplemented!()
}
