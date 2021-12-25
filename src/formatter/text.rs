use formatter;
use task;

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Formatter {
        return Formatter {};
    }
}

impl formatter::Formats for Formatter {
    fn formatted(&self, task: task::Task) -> String {
        let mut out = String::from("");

        out += &format!(
            "Task {:?} ({:?}): {}\n",
            &task.source.kind, &task.severity, &task.name
        )
        .to_string();

        out += &format!("[Found on {}:{}:{}]",
                        &task.source.path,
            &task.source.line,
            &task.source.column
        ).to_string();

        out += &match &task.context {
            Some(ctx) => format!("\n{}", &ctx.body()),
            None => String::from(""),
        };

        return out;
    }
    fn get_delimiter(&self, delim: formatter::Delimiter, point: formatter::Point) -> String {
        match delim {
            formatter::Delimiter::TASK => match point {
                formatter::Point::END => String::from("\n"),
                _ => String::from(""),
            },
            formatter::Delimiter::SECTION => match point {
                _ => String::from(""),
            },
        }
    }
    fn get_title(&self, title: String) -> String {
        String::from("")
    }
}
