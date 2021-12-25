mod mock;
use mock::CODE;
mod pattern;
mod task;
mod formatter;

fn main() {
    let additional_types = vec![String::from("custom")];
    let f = formatter::Format::new(formatter::Format::TEXT);
    for kind in task::Type::list_with_additional(Some(additional_types)) {
        f.start_section(kind.target());

        let m = pattern::Matcher::new(kind);
        let mut ext = pattern::Extractor::new(m, CODE.to_string());

        loop {
            match ext.get_task() {
                Some(task) => {
                    f.delimiter(formatter::Delimiter::TASK, formatter::Point::START);
                    f.render(task);
                    f.delimiter(formatter::Delimiter::TASK, formatter::Point::END);
                },
                None => break,
            };
        }
        f.end_section();
    }
}
