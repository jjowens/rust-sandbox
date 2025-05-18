use rust_sandbox::services::{book::*, author::*};
use rust_sandbox::helpers::print_helper::*;

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

        let actual_result =  print_book(Some(&_book1)).to_string();
        let expected_result = "Salem's Lot - Stephen King".to_string();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_print_nothing_if_no_book() {
        let actual_result =  print_book(None).to_string();
        let expected_result = "".to_string();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_print_author() {
        let _author = Author {
            first_name: "Ray".to_string(),
            last_name: "Bradbury".to_string(),
        };

        let actual_result =  print_author(Some(&_author)).to_string();
        let expected_result = "Ray Bradbury".to_string();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_print_nothing_if_no_author() {
        let actual_result =  print_author(None).to_string();
        let expected_result = "".to_string();

        assert_eq!(actual_result, expected_result);
    }

}