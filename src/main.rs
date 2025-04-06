use rust_sandbox::shop::{book::*, author::*};
use rust_sandbox::helpers::print_helper::*;

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
