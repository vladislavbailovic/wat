use task;
use formatter;

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Formatter {
        return Formatter{ }
    }
}

impl formatter::Formats for Formatter {
    fn formatted(&self, task: task::Task) -> String {
        let mut out = String::from("");

        out += &format!("Task {:?} ({:?}): {}\n",
            &task.source.kind,
            &task.severity,
            &task.name).to_string();

        out += &format!("[Found on {}:{}]",
            &task.source.line,
            &task.source.column).to_string();

        out += &match &task.context {
            Some(ctx) => format!("{}", &ctx.body()),
            None => String::from(""),
        };

        return out;
    }
    fn get_delimiter(&self, delimiter: formatter::Delimiter) -> String {
        match delimiter {
            formatter::Delimiter::TASK => String::from("--------------------\n"),
            formatter::Delimiter::SECTION => String::from("===\n"),
        }
    }
}
