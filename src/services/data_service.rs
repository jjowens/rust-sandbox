use crate::helpers::book_helper::build_book;
use crate::services::book::Book;

pub fn get_books() -> Vec<Book> {
    let mut vec = vec![];

    vec.push(build_book("IT".to_string(), "Stephen".to_string(), "King".to_string()));
    vec.push(build_book("The Empire State".to_string(), "Colin".to_string(), "Bateman".to_string()));
    vec.push(build_book("Romantic Comedy".to_string(), "Curtis".to_string(), "Steiger".to_string()));

    vec
}