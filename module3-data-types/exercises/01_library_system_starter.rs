// Define the Book struct
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u32,
    isbn: String,
}

impl Book {
    fn new(title: &str, author: &str, year: u32, isbn: &str) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            isbn: isbn.to_string(),
        }
    }
}

// Define a BookStatus enum to track availability
enum BookStatus {
    Available,
    Borrowed,
}

// Define a Library struct to manage books
struct Library {
    books: Vec<(Book, BookStatus)>,
}

// Implement methods for the Library struct
impl Library {
    // Create a new, empty library
    fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        self.books.push((book, BookStatus::Available));
    }

    // Borrow a book from the library
    fn borrow_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        for (book, status) in &mut self.books {
            if book.isbn == isbn {
                match status {
                    BookStatus::Available => {
                        *status = BookStatus::Borrowed;
                        return Ok(book);
                    }
                    BookStatus::Borrowed => {
                        return Err("Book is already borrowed");
                    }
                }
            }
        }
        Err("Book not found")
    }

    // Return a borrowed book to the library
    fn return_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        for (book, status) in &mut self.books {
            if book.isbn == isbn {
                match status {
                    BookStatus::Borrowed => {
                        *status = BookStatus::Available;
                        return Ok(book);
                    }
                    BookStatus::Available => {
                        return Err("Book is not borrowed");
                    }
                }
            }
        }
        Err("Book not found")
    }

    // List all books in the library with their status
    fn list_books(&self) {
        println!("Library Book Inventory:");
        for (book, status) in &self.books {
            let status_str = match status {
                BookStatus::Available => "Available",
                BookStatus::Borrowed => "Borrowed",
            };
            println!("\"{}\" by {} ({}) - {}", book.title, book.author, book.year, status_str);
        }
        println!();
    }
}

fn main() {
    // Create a new library
    let mut library = Library::new();
    
    // Add several books to the library
    library.add_book(Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        2018,
        "9781718500440"
    ));
    
    library.add_book(Book::new(
        "Design Patterns",
        "Erich Gamma et al.",
        1994,
        "9780201633610"
    ));
    
    library.add_book(Book::new(
        "Clean Code",
        "Robert C. Martin",
        2008,
        "9780132350884"
    ));
    
    // List all books
    library.list_books();
    
    // Borrow a book
    println!("Borrowing \"Clean Code\"...");
    match library.borrow_book("9780132350884") {
        Ok(_) => println!("Book borrowed successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books again to see the updated status
    library.list_books();
    
    // Return the book
    println!("Returning \"Clean Code\"...");
    match library.return_book("9780132350884") {
        Ok(_) => println!("Book returned successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books one more time
    library.list_books();
}