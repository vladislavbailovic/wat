use task;
use formatter;

pub struct Formatter {
    task: task::Task
}

impl Formatter {
    pub fn new(task: task::Task) -> Formatter {
        return Formatter{ task }
    }
}

impl formatter::Formats for Formatter {
    fn formatted(&self) -> String {
        let mut out = String::from("");

        out += &format!("Task {:?} ({:?}): {}\n",
            &self.task.source.kind,
            &self.task.severity,
            &self.task.name).to_string();

        out += &format!("[Found on {}:{}]",
            &self.task.source.line,
            &self.task.source.column).to_string();

        out += &match &self.task.context {
            Some(ctx) => format!("{}", &ctx.body()),
            None => String::from(""),
        };

        return out;
    }
}
