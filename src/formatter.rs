mod text;
mod html;
use task;

pub trait Renders {
    fn render(&self, task: task::Task);
    fn delimiter(&self, delimiter: Delimiter, point: Point);
}

pub trait Formats {
    fn formatted(&self, task: task::Task) -> String;
    fn get_delimiter(&self, delimiter: Delimiter, point: Point) -> String;
}

impl<T> Renders for T where T: Formats {
    fn render(&self, task: task::Task) {
        let out = self.formatted(task);
        if out.len() > 0 {
            println!("{}", &out);
        }
    }
    fn delimiter(&self, delimiter: Delimiter, point: Point) {
        let out = self.get_delimiter(delimiter, point);
        if out.len() > 0 {
            println!("{}", &out);
        }
    }
}

pub enum Format {
    TEXT,
    HTML,
}

impl Format {
    pub fn new(kind: Format) -> Box<dyn Renders> {
        match kind {
            Format::TEXT => Box::new(text::Formatter::new()),
            Format::HTML => Box::new(html::Formatter::new()),
        }
    }
}

pub enum Delimiter {
    TASK,
    SECTION,
}

pub enum Point {
    START,
    END,
}
