mod shop;
pub use shop::book;
pub use shop::author;
use crate::book::Book;
use crate::author::Author;

fn main() {
    let _book1 = Book {
        title: "Salem's Lot".to_string(),
        author: Author {
            first_name: "Stephen".to_string(),
            last_name: "King".to_string(),
        }
    };
}
