#[derive(Debug, Clone)]
pub enum Type {
	TODO,
	FIXME,
	Custom(String)
}

#[derive(Debug)]
pub enum Severity {
	URGENT,
	HIGH,
	NORMAL,
	Custom(i8),
}

impl Severity {
    pub fn new(order: usize) -> Severity {
        match order {
			0 => Severity::NORMAL,
			1 => Severity::HIGH,
			2 => Severity::URGENT,
			n => Severity::Custom(n as i8),
        }
    }
}

#[derive(Debug)]
pub struct Task {
   pub name: String,
   pub source: TaskSource,
   pub severity: Severity,
   pub context: Option<Context>,
}


#[derive(Debug)]
pub struct Context {
    pub body: Vec<String>,
}

#[derive(Debug)]
pub struct TaskSource {
	pub kind: Type,
	pub line: usize,
	pub column: usize,
}
