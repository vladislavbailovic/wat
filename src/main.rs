mod mock;
use mock::CODE;
mod task;


#[derive(Debug)]
struct Matcher {
	kind: task::Type,
	target: String,
	situational: String,
	severity: String,
	comment_pattern: String,
}

impl Matcher {
	fn new(kind: task::Type) -> Matcher {
		let situational = String::from(":");
		let severity = String::from("!");
		let comment_pattern = String::from("//");
		let target = match &kind {
			task::Type::TODO => "TODO".to_string(),
			task::Type::FIXME => "FIXME".to_string(),
			task::Type::Custom(tgt) => tgt.to_string(),
		};
		return Matcher{ kind, target, severity, situational, comment_pattern };
	}
}

struct Extractor {
	mt: Matcher,
	code: String,
	lines: Vec<String>,
	pos: usize,
}

impl Extractor {
	fn new(mt: Matcher, code: String) -> Extractor {
        let mut lines: Vec<String> = vec![];
		let raw: Vec<&str> = code.split("\n").collect();
		for line in raw {
			lines.push(line.to_string());
		}
		Extractor{ mt, code, lines, pos: 0 }
	}

	fn get_task(&mut self) -> Option<task::Task> {
		if !self.code.contains(&self.mt.target) {
			return None;
		}

		while self.pos < self.lines.len() {
            let task = self.process_line();
            self.pos += 1;
            if let None = task {
                continue;
            }
            return task;
		}
        None
	}

	fn process_line(&mut self) -> Option<task::Task> {
		let line = self.lines.get(self.pos)
			.expect("There should be a line there");
		if !line.contains(&self.mt.target) {
			return None;
		}

		let target_len = self.mt.target.chars().count();
		let byte_pos = line.find(&self.mt.target)
			.expect("Unable to find byte position of the first match");
		let after = (&line[byte_pos+target_len..]).trim();
		let source = task::TaskSource{
			kind: self.mt.kind.clone(),
			line: self.pos,
			column: byte_pos as usize,
		};

		let name = self.determine_task_name(&after);
		let severity = self.determine_severity(&after);

		let task = task::Task{
			source, name, severity,
            context: self.process_context(),
		};

        Some(task)
	}

	fn process_context(&mut self) -> Option<task::Context> {
		let next_line = self.lines.get(self.pos+1)
			.expect("There should be a next line");
		let mut context: Vec<String> = vec![];
		if self.mt.comment_pattern == next_line.trim() {
			// Hit empty comment line: context delimiter.
			// Pick up everything until the end of the comment.
			self.pos += 2; // consume the delimiter line, start at line after.
			while self.pos < self.lines.len() {
				let raw = self.lines.get(self.pos).expect("context line").trim();
				let sans = raw.trim_start_matches(&self.mt.comment_pattern);
				if raw.len() == sans.len() {
					break;
				}
				context.push(sans.trim().to_string());
				self.pos += 1;
			}
		} else { return None; }
		Some(task::Context{ body: context })
	}

	fn determine_task_name(&self, line: &str) -> String {
		let mut idx = 0;
		while idx < line.len() {
			let c = &line[idx..idx+1];
			if c != self.mt.situational && c != " " && c != self.mt.severity {
				break;
			}
			idx += 1;
		}
		return String::from(&line[idx..]);
	}

	fn determine_severity(&self, line: &str) -> task::Severity {
		let nosvt = &line.trim_start_matches(&self.mt.severity);
        task::Severity::new(line.len() - nosvt.len())
	}
}

fn main() {
	let m = Matcher::new(task::Type::TODO);
	let mut ext = Extractor::new(m, CODE.to_string());
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
