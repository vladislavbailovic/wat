use task;
use formatter;

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Formatter {
        return Formatter{};
    }
}

impl formatter::Formats for Formatter {
    fn formatted(&self, task: task::Task) -> String {
        let mut out = String::from("");

        out += &(
            String::from("\t<header>\n") +
            &format!("\t\t<h1>{}</h1>\n", &task.name) +

            &String::from("\t\t<dl>\n") +
            &format!("\t\t\t<dt>Type<dt><dd>{:?}</dd>\n", &task.source.kind) +
            &format!("\t\t\t<dt>Severity<dt><dd>{:?}</dd>\n", &task.severity) +
            &String::from("\t\t</dl>\n") +

            &String::from("\t\t<dl>\n") +
            &format!("\t\t\t<dt>Line<dt><dd>{:?}</dd>\n", &task.source.line) +
            &format!("\t\t\t<dt>Char<dt><dd>{:?}</dd>\n", &task.source.column) +
            &String::from("\t\t</dl>\n") +

            &String::from("\t</header>\n"));

        out += &match &task.context {
            Some(ctx) => {
                let body = ctx.body();
                let lines: Vec<&str> = body.split("\n").collect();
                format!("\t<div>\n\t\t<p>{}</p>\n\t</div>\n", &lines.join("</p><p>"))
            },
            None => String::from(""),
        };

        return out;
    }
    fn get_delimiter(&self, delim: formatter::Delimiter, point: formatter::Point) -> String {
        match delim {
            formatter::Delimiter::TASK => match point {
                formatter::Point::START => String::from("<article>\n"),
                formatter::Point::END => String::from("</article>\n"),
            },
            formatter::Delimiter::SECTION => match point {
                formatter::Point::START => String::from("<section>\n"),
                formatter::Point::END => String::from("</section>\n"),
            },
        }
    }
}
