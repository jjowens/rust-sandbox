#[cfg(test)]
mod data_service_test {
    use rust_sandbox::services::data_service::{*};

    #[test]
    fn test_get_books() {
        //let _expected_results = vec![1,2,3,4,5];
        let _actual_results = get_books();

        assert!(_actual_results.len() > 0);

        for book in _actual_results {
            println!("{}", book.to_string())
        }
    }
}

