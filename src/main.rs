mod mock;
use mock::CODE;
mod pattern;
mod task;

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
            println!(
                "\tTask {:?}({:?}): {}",
                &task.source.kind, &task.severity, &task.name
            );
            println!("\t[Found on {}:{}]", &task.source.line, &task.source.column);
            match task.context {
                Some(ctx) => println!("\t{}", ctx.body()),
                None => (),
            };
            println!("\t--------------------\n");
        }
        println!("====================\n");
    }
}
