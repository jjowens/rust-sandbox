use crate::shop::book::Book;
use crate::shop::author::Author;

pub fn print_book(_book: Option<Book>) -> String {
    match _book {
        Some(item) => format!(
            "{} - {} {}",
            item.title.to_string(),
            item.author.first_name.to_string(),
            item.author.last_name.to_string()
        ),
        None => "".to_string(),
    }
}

pub fn print_author(_author: Option<Author>) -> String {
    match _author {
        Some(item) => format!(
            "{} {}",
            item.first_name.to_string(),
            item.last_name.to_string()
        ),
        None => "".to_string(),
    }
}
