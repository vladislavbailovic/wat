pub const SEVERITY: &str = "!";
pub const SITUATIONAL: &str = ":";

pub trait Target {
    fn target(&self) -> String;
}

pub enum CommentStyle {
    Slc,
    Shell,
    Sql,
    Custom(String),
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
