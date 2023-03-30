
pub fn get_sha1(value: &str) -> String {
    let mut hasher = sha1_smol::Sha1::new();
    hasher.update(value.as_bytes());
    hasher.digest().to_string()
}