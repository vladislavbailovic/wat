mod mock;
use mock::CODE;

#[derive(Debug, Clone)]
enum TaskType {
	TODO,
	FIXME,
	Custom(String)
}

#[derive(Debug)]
enum TaskSeverity {
	URGENT,
	HIGH,
	NORMAL,
	Custom(i8),
}

#[derive(Debug)]
struct Task {
    name: String,
	source: TaskSource,
	severity: TaskSeverity,
    context: Option<TaskContext>,
}

#[derive(Debug)]
struct TaskContext {
    body: Vec<String>,
}

#[derive(Debug)]
struct TaskSource {
	kind: TaskType,
	line: usize,
	column: usize,
}

#[derive(Debug)]
struct Matcher {
	kind: TaskType,
	target: String,
	prefix: String,
	situational: String,
	severity: String,
	comment_pattern: String,
}

impl Matcher {
	fn new(kind: TaskType) -> Matcher {
		let prefix = String::from("@");
		let situational = String::from(":");
		let severity = String::from("!");
		let comment_pattern = String::from("//");
		let target = match &kind {
			TaskType::TODO => "TODO".to_string(),
			TaskType::FIXME => "FIXME".to_string(),
			TaskType::Custom(tgt) => tgt.to_string(),
		};
		return Matcher{ kind, target, prefix, severity, situational, comment_pattern };
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
		Extractor{ mt, code, lines: vec![ String::from("") ], pos: 0 }
	}

	fn get_task(&mut self) -> Option<Task> {
		if !self.code.contains(&self.mt.target) {
			return None;
		}

		let lines: Vec<&str> = self.code.split("\n").collect();
		for line in lines {
			self.lines.push(line.to_string());
		}
		while self.pos < self.lines.len() {
			self.process_line();
			self.pos += 1;
		}
		None
	}

	fn process_line(&mut self) {
		let line = self.lines.get(self.pos)
			.expect("There should be a line there");
		if !line.contains(&self.mt.target) {
			return;
		}

		let target_len = self.mt.target.chars().count();
		let byte_pos = line.find(&self.mt.target)
			.expect("Unable to find byte position of the first match");
		let before = (&line[0..byte_pos]).trim();
		let after = (&line[byte_pos+target_len..]).trim();
		let source = TaskSource{
			kind: self.mt.kind.clone(),
			line: self.pos,
			column: byte_pos as usize,
		};

		let name = self.determine_task_name(&after);
		let severity = self.determine_severity(&after);

		let task = Task{
			source, name, severity,
            context: self.process_context(),
		};

		dbg!(&task);
	}

	fn process_context(&mut self) -> Option<TaskContext> {
		let nextLine = self.lines.get(self.pos+1)
			.expect("There should be a next line");
		let mut context: Vec<String> = vec![];
		if self.mt.comment_pattern == nextLine.trim() {
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
			// self.pos += ctxLine;
		} else { return None; }
		Some(TaskContext{ body: context })
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

	fn determine_severity(&self, line: &str) -> TaskSeverity {
		let nosvt = &line.trim_start_matches(&self.mt.severity);
		match line.len() - nosvt.len() {
			0 => TaskSeverity::NORMAL,
			1 => TaskSeverity::HIGH,
			2 => TaskSeverity::URGENT,
			n => TaskSeverity::Custom(n as i8),
		}
	}
}

fn main() {
	let m = Matcher::new(TaskType::TODO);
	let mut ext = Extractor::new(m, CODE.to_string());
	loop {
		let task = match ext.get_task() {
			Some(t) => t,
			None => break,
		};
		dbg!(task);
	}
}
