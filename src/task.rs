use parser::Target;

#[derive(Debug, Clone)]
pub enum Type {
    TODO,
    FIXME,
    Custom(String),
}

impl Type {
    pub fn list() -> Vec<Type> {
        return vec!( Type::TODO, Type::FIXME );
    }

    pub fn list_with_additional(custom: Option<Vec<String>>) -> Vec<Type> {
        let mut res = Type::list();
        let custom = match custom {
            Some(kinds) => kinds,
            None => vec![],
        };
        for additional in custom {
            res.push(Type::Custom(additional));
        }
        return res;
    }

    pub fn kind(target: &str) -> Type {
        match target{
            "TODO" => Type::TODO,
            "FIXME" => Type::FIXME,
            kind => Type::Custom(kind.to_string()),
        }
    }
}

impl Target for Type {
    fn target(&self) -> String {
        match self {
            Type::TODO => String::from("TODO"),
            Type::FIXME => String::from("FIXME"),
            Type::Custom(tgt) => tgt.to_string(),
        }
    }
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
    pub source: Source,
    pub severity: Severity,
    pub context: Option<Context>,
}

#[derive(Debug)]
pub struct Context {
    raw: Vec<String>,
}

impl Context {
    pub fn new(raw: Vec<String>) -> Context {
        Context { raw }
    }

    pub fn body(&self) -> String {
        let mut body: Vec<String> = vec![String::from("")];
        let mut idx = 0;
        for line in &self.raw {
            if "" == line.trim() {
                body.push(String::from(""));
                idx += 1;
                continue;
            }
            body[idx] += &(line.to_owned() + " ");
        }
        return body.join("\n");
    }
}

#[derive(Debug)]
pub struct Source {
    pub kind: Type,
    pub line: usize,
    pub column: usize,
}
