mod mock;
use mock::CODE;
mod pattern;
mod task;
mod formatter;

fn main() {
    let additional_types = vec![String::from("custom")];
    let f = formatter::Format::new(formatter::Format::TEXT);
    for kind in task::Type::list_with_additional(Some(additional_types)) {
        println!("Finding {}", task::Type::target(&kind));
        f.delimiter(formatter::Delimiter::SECTION, formatter::Point::START);

        let m = pattern::Matcher::new(kind);
        let mut ext = pattern::Extractor::new(m, CODE.to_string());

        loop {
            f.delimiter(formatter::Delimiter::TASK, formatter::Point::START);
            let task = match ext.get_task() {
                Some(t) => t,
                None => break,
            };
            f.render(task);
            f.delimiter(formatter::Delimiter::TASK, formatter::Point::END);
        }
        f.delimiter(formatter::Delimiter::SECTION, formatter::Point::END);
    }
}
