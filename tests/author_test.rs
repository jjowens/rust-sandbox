#[cfg(test)]
mod author_test {
    use rust_sandbox::services::author::Author;

    #[test]
    fn should_print_author() {
        let _author = Author {
            first_name: "Charles".to_string(),
            last_name: "Dickens".to_string(),
        };

        let expected_result = "Charles Dickens".to_string();
        let actual_result = _author.to_string();

        assert_eq!(actual_result, expected_result);

    }

}