use rust_sandbox::services::data_service::get_books;

fn main() {

    let books = get_books();

    for book in books {
        println!("{}", book.to_string())
    }

}
