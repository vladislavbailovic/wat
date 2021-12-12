mod mock;
use mock::CODE;
mod pattern;
mod task;
mod formatter;

fn main() {
    let additional_types = vec![String::from("custom")];
    for kind in task::Type::list_with_additional(Some(additional_types)) {
        println!("Finding {}", task::Type::target(&kind));
        let m = pattern::Matcher::new(kind);
        let mut ext = pattern::Extractor::new(m, CODE.to_string());
        loop {
            let task = match ext.get_task() {
                Some(t) => t,
                None => break,
            };
            let f = formatter::Format::new(formatter::Format::TEXT, task);
            f.render();
            println!("--------------------\n");
        }
        println!("====================\n");
    }
}
