use crate::shop::author::Author;

#[derive(Clone)]
pub struct Book {
    pub title: String,
    pub author: Author
}

impl Book {
    pub fn to_string(&self) -> String {
        format!(
            "{} - {} {}",
            self.title.to_string(),
            self.author.first_name.to_string(),
            self.author.last_name.to_string()
        )
    }
}