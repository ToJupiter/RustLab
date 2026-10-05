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
    pub model: ChatGPTModels,
    pub reasoning_level: u8,
    pub prompt: String
}

#[derive(Debug)]
enum ChatGPTModels {
    Chatgpt4,
    Chatgpt5,
    Chatgpt55,
    Chatgpt6
}

impl Study for ChatGPT {
    fn read_books(&self) {
        println!("ChatGPT is studying based on this book!");
    }

    fn write_to_notebooks(&self) {
        println!("ChatGPT model {:?} and reasoning level {} is writing a notebook for its user to read", self.model, self.reasoning_level);
    }
}

pub struct Claude {
    pub model: ClaudeModels,
    pub reasoning_level: u8,
    pub prompt: String
}

#[derive(Debug)]
enum ClaudeModels {
    OPUS,
    FABLE,
    MYTHOS,
    SONNET,
    HAIKU
}

impl Study for Claude {
    fn read_books(&self) {
        println!("Anthropic is studying based on this book!");
    }

    fn write_to_notebooks(&self) {
        println!("Claude model {:?} and reasoning level {} is writing a notebook for its user to read", self.model, self.reasoning_level);
    }
}


pub fn ai_models_for_studying() {
    let student_studies_literature = Student {
        books_to_read: vec![
            Box::new(Claude {
                prompt: String::from("Generate me the a Haiku style poem"),
                model: ClaudeModels::HAIKU,
                reasoning_level: 4
            }),
            Box::new(ChatGPT {
                prompt: String::from("Generate me a poem that is better than Claude does"),
                model: ChatGPTModels::Chatgpt6,
                reasoning_level: 5
            })
        ],
        notebooks_to_write: vec![
            Box::new(Claude {
                prompt: String::from("Write all content of the lecture on modern literature into my virtual notebook"),
                model: ClaudeModels::SONNET,
                reasoning_level: 4
            }),
            Box::new(ChatGPT {
                prompt: String::from("Write all content of the 16th century lecture on Chinese literature into my virtual notebook"),
                model: ChatGPTModels::Chatgpt6,
                reasoning_level: 5
            })
        ]
    };
}