pub trait Study {
    fn read_books(&self);
    fn write_to_notebooks(&self);
}

pub struct Student {
    pub books_to_read: Vec<Box<dyn Study>>,
    pub notebooks_to_write: Vec<Box<dyn Study>>
}

impl Student {
    pub fn student_reads_the_book(&self) {
        for book in self.books_to_read.iter() {
            book.read_books();
        }
    } 
}

pub struct ChatGPT {
    pub model: String,
    pub reasoning_level: u8
}

impl Study for ChatGPT {
    fn read_books(&self) {
        println!("ChatGPT is studying based on this book!");
    }

    fn write_to_notebooks(&self) {
        println!("ChatGPT model {} and reasoning level {} is writing a notebook for its user to read", self.model, self.reasoning_level);
    }
}

