mod mock;
use mock::CODE;
mod pattern;
mod task;

fn main() {
    let m = pattern::Matcher::new(task::Type::TODO);
    let mut ext = pattern::Extractor::new(m, CODE.to_string());
    loop {
        let task = match ext.get_task() {
            Some(t) => t,
            None => break,
        };
        println!(
            "Task {:?}({:?}): {}",
            &task.source.kind, &task.severity, &task.name
        );
        println!("[Found on {}:{}]", &task.source.line, &task.source.column);
        match task.context {
            Some(ctx) => println!("{}", ctx.body()),
            None => (),
        };
        println!("--------------------\n");
    }
}
