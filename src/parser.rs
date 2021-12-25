pub const SEVERITY: &str = "!";
pub const SITUATIONAL: &str = ":";

pub trait Target {
    fn target(&self) -> String;
}

#[derive(Debug)]
pub enum CommentStyle {
    Slc,
    Shell,
    Sql,
    Custom(String),
}

impl CommentStyle {
    pub fn from_file(ft: &FileType) -> CommentStyle {
        match ft {
            FileType::JavaScript => CommentStyle::Slc,
            FileType::Rust => CommentStyle::Slc,
            FileType::Go => CommentStyle::Slc,
            FileType::Php => CommentStyle::Slc,
            FileType::Python => CommentStyle::Shell,
            FileType::Shell => CommentStyle::Shell,
            FileType::Sql => CommentStyle::Sql,
        }
    }
}

impl Target for CommentStyle {
    fn target(&self) -> String {
        match self {
            CommentStyle::Slc => String::from("//"),
            CommentStyle::Shell => String::from("#"),
            CommentStyle::Sql => String::from("--"),
            CommentStyle::Custom(m) => m.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum FileType {
    JavaScript,
    Rust,
    Go,
    Php,
    Python,
    Shell,
    Sql,
}

#[derive(Debug)]
pub struct File {
    pub kind: FileType,
    pub path: String
}
