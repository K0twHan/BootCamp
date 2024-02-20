

enum Publication {
        Book(Book),
        Magazine(Magazine)
    }
    struct Book {
        title : String,
        author : String,
        page_count : u8,
    }
    struct Magazine {
        title : String,
        issue : u8,
        topic : String,
    }
    fn print_my_vec(publications: Vec<Publication>)
    {
        for publication in publications {
            match publication {
                Publication::Book(ref book) => {
                    println!("book: {}, author {} , page_count {}",book.title,book.author,book.page_count)
                }
                Publication::Magazine(ref magazine) => {
                    println!("magazine : {}, issue : {}, topic : {}",magazine.title,magazine.issue,magazine.topic)
                }
            }
        }
    }

    fn main() {
        let book_first = Book {
            title : "BookExapmle".to_string(),
            author : "J.J. Hanry".to_string(),
            page_count : 20
        };
        let magazine_first = Magazine {
            title : "MagazineExample".to_string(),
            issue : 40,
            topic : "ExampleTopic".to_string()
        };

        let publications = vec![Publication::Book(book_first),Publication::Magazine(magazine_first)];
        print_my_vec(publications)


}
