use std::fs;

use crate::http::{Methods, Response};

use super::server::Handler;
pub struct WebsiteHandler {
    public_folder_path: String,
}

impl WebsiteHandler {
    pub fn new(public_folder_path: String) -> Self {
        Self { public_folder_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_folder_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_folder_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Someone tried a Traversal attack! path {}", path.display());
                    None
                }
            }
            Err(_) => None
        }

    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        match request.method() {
            Methods::GET => match request.path() {
                "/" => Response::new(crate::http::StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(
                    crate::http::StatusCode::Ok,
                    Some("<h1> Hello world!</h1>".to_string()),
                ),
                path => match self.read_file(path) {
                    Some(content) => Response::new(crate::http::StatusCode::Ok, Some(content)),
                    None => Response::new(crate::http::StatusCode::Notfound, None),
                },
            },
            _ => Response::new(
                crate::http::StatusCode::Ok,
                Some("<h1> Hello! </h1>".to_string()),
            ),
        }
    }
}
