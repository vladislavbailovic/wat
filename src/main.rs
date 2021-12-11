#[derive(Debug)]
enum TaskType {
	TODO,
	FIXME,

	Custom(String)
}

#[derive(Debug)]
struct Task {
    name: String,
	source: TaskSource,
}

#[derive(Debug)]
struct TaskSource {
	kind: TaskType,
	line: i32,
	column: i32,
}

#[derive(Debug)]
struct Matcher {
	kind: TaskType,
	target: String,
	prefix: String,
	situational: String,
	severity: char,
}

impl Matcher {
	fn new(kind: TaskType) -> Matcher {
		let prefix = String::from("@");
		let situational = String::from(":");
		let severity = '!';
		let target = match &kind {
			TaskType::TODO => "TODO".to_string(),
			TaskType::FIXME => "FIXME".to_string(),
			TaskType::Custom(tgt) => tgt.to_string(),
		};
		return Matcher{ kind, target, prefix, severity, situational };
	}

	fn get_task(&self, code: String) -> Option<Task> {
		if !code.contains(&self.target) {
			return None;
		}
		let mut idx = -1;
		let target_len = self.target.chars().count();
		let mut comment_pattern = String::from("");
		for line in code.split("\n") {
			idx += 1;
			if !line.contains(&self.target) {
				continue;
			}
			let byte_pos = line.find(&self.target)
				.expect("Unable to find byte position of the first match");
			let before = (&line[0..byte_pos]).trim();
			let after = (&line[byte_pos+target_len..]).trim();
			let source = TaskSource{
				kind: match &self.kind {
					TaskType::TODO => TaskType::TODO,
					TaskType::FIXME => TaskType::FIXME,
					TaskType::Custom(n) => TaskType::Custom(n.to_string()),
				},
				line: idx,
				column: byte_pos as i32,
			};

			if comment_pattern.len() == 0 {
				dbg!("determining comment pattern");
				comment_pattern = self.determine_comment_pattern(&before);
			}
			let name = self.determine_task_name(&after);
			// let severity = self.determineSeverity(&before);

			let task = Task{
				source,
				name,
			};
			dbg!(&task);
			
		};
		None
	}

	fn determine_comment_pattern(&self, line: &str) -> String {
		let mut count = line.len();
		let mut ptn = String::from("");
		while count > 0 {
			count -= 1;
			let c = &line[count..count+1];
			if c == self.prefix {
				while &line[count-1..count] == " " {
					count -= 1;
				}
				continue;
			}
			if ptn.len() >= 1 && c.trim().len() < 1 {
				break;
			}
			ptn += c;
		}
		return String::from(ptn);
	}	

	fn determine_task_name(&self, line: &str) -> String {
		let mut idx = 0;
		while idx < line.len() {
			let c = &line[idx..idx+1];
			if c != self.situational && c != " " {
				break;
			}
			idx += 1;
		}
		return String::from(&line[idx..]);
	}
}

fn main() {
	let m = Matcher::new(TaskType::TODO);
	loop {
		let task = match m.get_task(CODE.to_string()) {
			Some(t) => t,
			None => break,
		};
		dbg!(task);
	}
}




const CODE: &str = "
mport Api from './api';

module.exports = context => {

        const request = context.request;

        const Cache = require( './cache' )( context );
        const Member = require( './member' )( context );

        return class Hub {

                static RQ_LIST_SITES = 'getSites';
                static RQ_GET_SITE = 'getSiteInfo';
                static RQ_PREPARE_PUSH = 'preparePush';
                static RQ_CLEANUP_PUSH = 'cleanupPush';

                constructor( site ) {
                        this.site = site;
                        this.member = new Member( site );
                }

                getSites() {
                        // @TODO remove stub!
                        return Promise.resolve(
                                [ 'body-exposure' ]
                        );
						// TODO: use this instead
                        return new Promise( ( resolve, reject ) => {
                                this.getData( Hub.RQ_LIST_SITES )
                                        .then( data => resolve( data || [] ) )
                                        .catch( reject )
                                ;
                        } );
                }
		}
	}
}";
