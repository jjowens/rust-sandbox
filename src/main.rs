mod helpers;
mod shop;
use crate::author::Author;
use crate::book::Book;
pub use helpers::print_helper::print_book;
pub use shop::author;
pub use shop::book;

fn main() {
    let _book1 = Book {
        title: "Salem's Lot".to_string(),
        author: Author {
            first_name: "Stephen".to_string(),
            last_name: "King".to_string(),
        },
    };

    println!("{}", print_book(Some(_book1)).to_string());
}
