mod mock;
use mock::CODE;
mod formatter;
mod parser;
mod pattern;
mod task;

use parser::Target;

fn main() {
    let additional_types = vec![String::from("custom")];
    let f = formatter::Format::new(formatter::Format::TEXT);
    let comment = parser::CommentStyle::Slc;
    for kind in task::Type::list_with_additional(Some(additional_types)) {
        f.start_section(kind.target());

        let mut ext = pattern::Extractor::new(&kind, &comment, CODE.to_string());

        loop {
            match ext.get_task() {
                Some(task) => {
                    f.delimiter(formatter::Delimiter::TASK, formatter::Point::START);
                    f.render(task);
                    f.delimiter(formatter::Delimiter::TASK, formatter::Point::END);
                }
                None => break,
            };
        }
        f.end_section();
    }
}
