#[cfg(test)]
mod book_helper_test {
    use rust_sandbox::helpers::book_helper::*;

    #[test]
    fn test_build_book() {

        let actual_result = build_book("The Crow Road".to_string(), "Ian".to_string(), "Banks".to_string());
        let expected_result = "The Crow Road - Ian Banks";

        assert_eq!(actual_result.to_string(), expected_result);
    }

}