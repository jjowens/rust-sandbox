use crate::services::book::Book;
use crate::services::author::Author;

pub fn build_book(title: String, first_name: String, last_name: String) -> Book {
    let book = Book {
        title,
        author: Author {
            first_name,
            last_name }
    };

    book
}