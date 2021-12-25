use task;

#[derive(Debug)]
pub enum MatcherType {
    Comment(String),
    Situational(String),
    Severity(String),
    Target(String),
}

const SEVERITY: &str = "!";
const SITUATIONAL: &str = ":";

#[derive(Debug)]
pub struct Matcher {
    kind: task::Type,
    target: String,
    comment_pattern: String,
}

impl Matcher {
    pub fn new(kind: task::Type) -> Matcher {
        Matcher {
            kind: kind.clone(),
            target: task::Type::target(&kind),
            comment_pattern: String::from("//"),
        }
    }
}

pub struct Extractor {
    mt: Matcher,
    code: String,
    lines: Vec<String>,
    pos: usize,
}

impl Extractor {
    pub fn new(mt: Matcher, code: String) -> Extractor {
        let mut lines: Vec<String> = vec![];
        let raw: Vec<&str> = code.split("\n").collect();
        for line in raw {
            lines.push(line.to_string());
        }
        Extractor {
            mt,
            code,
            lines,
            pos: 0,
        }
    }

    pub fn get_task(&mut self) -> Option<task::Task> {
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
        let line = self
            .lines
            .get(self.pos)
            .expect("There should be a line there");
        if !line.contains(&self.mt.target) {
            return None;
        }

        let target_len = self.mt.target.chars().count();
        let byte_pos = line
            .find(&self.mt.target)
            .expect("Unable to find byte position of the first match");
        let after = (&line[byte_pos + target_len..]).trim();
        let source = task::Source {
            kind: self.mt.kind.clone(),
            line: self.pos,
            column: byte_pos as usize,
        };

        let name = self.determine_task_name(&after);
        let severity = self.determine_severity(&after);

        let task = task::Task {
            source,
            name,
            severity,
            context: self.process_context(),
        };

        Some(task)
    }

    fn process_context(&mut self) -> Option<task::Context> {
        let next_line = self
            .lines
            .get(self.pos + 1)
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
        } else {
            return None;
        }
        Some(task::Context::new(context))
    }

    fn determine_task_name(&self, line: &str) -> String {
        let mut idx = 0;
        while idx < line.len() {
            let c = &line[idx..idx + 1];
            if c != SITUATIONAL && c != " " && c != SEVERITY {
                break;
            }
            idx += 1;
        }
        return String::from(&line[idx..]);
    }

    fn determine_severity(&self, line: &str) -> task::Severity {
        let nosvt = &line.trim_start_matches(&SEVERITY);
        task::Severity::new(line.len() - nosvt.len())
    }
}
