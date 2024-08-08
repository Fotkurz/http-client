use reqwest;
use serde::Serialize;
use std::{fmt, io::Read};

pub struct Request {
    pub url: String,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(url={})", self.url)
    }
}

#[derive(Serialize)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

pub fn send(req: &Request) -> Response {
    let mut res = reqwest::blocking::get(req.url.clone()).unwrap();
    let mut body = String::new();

    res.read_to_string(&mut body).unwrap();

    println!("{}", body);

    Response {
        status: res.status().as_u16(),
        body: body,
    }
}
