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

#[cfg(test)]
mod tests{
    #[test]
    fn test_get_md5(){
        assert_eq!(crate::srun::md5::get_md5("15879684798qq","711ab370231392679fe06523b119a8fe096f5ed9bd206b4de8d7b5b994bbc3e5"), "b7cc5da95734d0161fadc8ad87855e75");
    }
}