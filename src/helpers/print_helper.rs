use crate::book::Book;

pub fn print_book(_book: Option<Book>) -> String {
    match _book {
        Some(item) => format!("{} - {} {}", item.title.to_string(),
                              item.author.first_name.to_string(),
                              item.author.last_name.to_string()),
        None => "".to_string()
    }
}