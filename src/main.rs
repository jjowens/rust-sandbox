mod shop;
mod helpers;
pub use shop::book;
pub use shop::author;
pub use helpers::print_helper::print_book;
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

    println!("{}", print_book(Some(_book1)).to_string());
}
