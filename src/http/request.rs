use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request_str = String::from_utf8_lossy(value); //convert bytes to string
        let mut lines = request_str.lines();
        let request_line = lines.next().ok_or(ParseError::InvalidRequest)?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next().ok_or(ParseError::InvalidRequest)?;
        let path = parts.next().ok_or(ParseError::InvalidRequest)?;
        let _protocol = parts.next().ok_or(ParseError::InvalidProtocol)?;
        let method = method.parse().map_err(|_| ParseError::InvalidMethod)?;

        let (path, query_string) = parse_path(path);

        Ok(Self {
            path,
            query_string,
            method,
        })

    }
    
}

// Function to separate path and query string
fn parse_path(path: &str) -> (String, Option<String>) {
    if let Some(pos) = path.find('?') {
        let (path, query) = path.split_at(pos);
        (path.to_string(), Some(query[1..].to_string()))
    } else {
        (path.to_string(), None)
    }
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {

}