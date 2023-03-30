use uuid::Uuid;
pub fn rand_str() -> String {
    let id = Uuid::new_v4();
    id.to_string()
}
