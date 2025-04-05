mod helpers;
use crate::shop;
use crate::author::Author;
use crate::book::Book;
pub use helpers::print_helper::print_book;

#[cfg(test)]
mod helper_test {
    use super::*;

    #[test]
    fn test_print_book() {
        let _book1 = Book {
            title: "Salem's Lot".to_string(),
            author: Author {
                first_name: "Stephen".to_string(),
                last_name: "King".to_string(),
            },
        };

        let actual_result =  print_book(Some(_book1)).to_string();
        let expected_result = "Salem's Lot - Stephen King".to_string();

        assert_eq!(actual_result, expected_result);
    }

}