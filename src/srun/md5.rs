use hmac::{Hmac, Mac};
use md5::Md5;

type HmacMD5 = Hmac<Md5>;

pub fn get_md5(password: &str, token: &str) -> String {
    let mut mac = HmacMD5::new_from_slice(token.as_bytes()).unwrap();
    mac.update(password.as_bytes());
    mac.finalize()
        .into_bytes()
        .iter()
        .map(|hex| format!("{:02x}", hex))
        .collect()
}
