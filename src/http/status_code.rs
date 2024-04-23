use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadResquest = 400,
    Notfound = 404
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "ok",
            Self::BadResquest => "Bad request",
            Self::Notfound => "Not found"
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}
