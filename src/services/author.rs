#[derive(Clone)]
pub struct Author {
    pub first_name: String,
    pub last_name: String
}
impl Author {
    pub fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.first_name.to_string(),
            self.last_name.to_string()
        )
    }
}