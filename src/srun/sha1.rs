
pub fn get_sha1(value: &str) -> String {
    let mut hasher = sha1_smol::Sha1::new();
    hasher.update(value.as_bytes());
    hasher.digest().to_string()
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_get_md5(){
        assert_eq!(crate::srun::sha1::get_sha1("123456"), "7c4a8d09ca3762af61e59520943dc26494f8941b");
    }
}