mod mock;
use mock::CODE;
mod task;
mod pattern;



fn main() {
	let m = pattern::Matcher::new(task::Type::TODO);
	let mut ext = pattern::Extractor::new(m, CODE.to_string());
	loop {
		let task = match ext.get_task() {
			Some(t) => t,
			None => break,
		};
        println!("Task {:?}({:?}): {}",
            &task.source.kind, &task.severity, &task.name);
        println!("[Found on {}:{}]", &task.source.line, &task.source.column);
        let ctx = match task.context {
            Some(ctx) => ctx.body,
            None => vec![],
        };
        for ln in ctx {
            println!("{}", ln);
        }
        println!("--------------------\n");
	}
}
