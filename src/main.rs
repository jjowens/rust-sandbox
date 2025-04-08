use rust_sandbox::shop::{book::*, author::*};
use rust_sandbox::helpers::print_helper::*;

fn main() {
    let _author = Author {
        first_name: "Stephen".to_string(),
        last_name: "King".to_string(),
    };

    let _book1 = Book {
        title: "Salem's Lot".to_string(),
        author: _author,
    };

    println!("{}", print_book(Some(&_book1)).to_string());
    println!("{}", _book1.to_string());

}
