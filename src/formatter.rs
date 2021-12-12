mod text;
use task;

pub trait Renders {
    fn render(&self);
}

pub trait Formats {
    fn formatted(&self) -> String;
}

impl<T> Renders for T where T: Formats {
    fn render(&self) {
        println!("{}", &self.formatted());
    }
}

pub enum Format {
    TEXT,
}
impl Format {
    pub fn new(kind: Format, task: task::Task) -> Box<dyn Renders> {
        match kind {
            _ => Box::new(text::Formatter::new(task)),
        }
    }
}
