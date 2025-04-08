#[cfg(test)]
mod author_test {
    use rust_sandbox::shop::author::Author;
    use rust_sandbox::shop::book::Book;

    #[test]
    fn should_print_book() {
        let _book = Book {
           title: "Night Watch".to_string(),
            author: Author {
                first_name: "Terry".to_string(),
                last_name: "Prachett".to_string()
            }
        };

        let expected_result = "Night Watch - Terry Prachett".to_string();
        let actual_result = _book.to_string();

        assert_eq!(actual_result, expected_result);

    }

}