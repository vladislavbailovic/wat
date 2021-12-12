mod text;
use task;

pub trait Renders {
    fn render(&self, task: task::Task);
    fn delimiter(&self, delimiter: Delimiter);
}

pub trait Formats {
    fn formatted(&self, task: task::Task) -> String;
    fn get_delimiter(&self, delimiter: Delimiter) -> String;
}

impl<T> Renders for T where T: Formats {
    fn render(&self, task: task::Task) {
        println!("{}", &self.formatted(task));
    }
    fn delimiter(&self, delimiter: Delimiter) {
        println!("{}", &self.get_delimiter(delimiter));
    }
}

pub enum Format {
    TEXT,
}

impl Format {
    pub fn new(kind: Format) -> Box<dyn Renders> {
        match kind {
            _ => Box::new(text::Formatter::new()),
        }
    }
}

pub enum Delimiter {
    TASK,
    SECTION,
}
