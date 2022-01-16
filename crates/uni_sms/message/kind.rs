use std::fmt;

#[derive(Debug, Clone)]
pub enum MessageKind {
    Custom(String),
    Template(String),
}

impl fmt::Display for MessageKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageKind::Custom(content) => write!(f, "{}", content),
            MessageKind::Template(template_id) => {
                write!(f, "{}", template_id)
            }
        }
    }
}
