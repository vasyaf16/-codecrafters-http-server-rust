use std::fmt::{Formatter};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    Empty,
    Echo(String),
    UserAgent(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    method: HttpMethod,
    path: String,
    command: Commands,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    header: Header,
    body: Option<String>,
}

impl Request {
    pub fn parse_request(stream: &mut TcpStream) -> anyhow::Result<Self> {
        let mut buf = BufReader::new(stream);
        let mut path = String::new();
        buf.read_line(&mut path)?;
        let mut host = String::new();
        buf.read_line(&mut host)?;
        let mut user_agent = String::new();
        buf.read_line(&mut user_agent)?;
        if let Some((method, rest)) = path.split_once(" ") {
            let method = match method {
                "GET" => HttpMethod::GET,
                "POST" => HttpMethod::POST,
                _ => return Err(anyhow!("Unknown Method"))
            };
            let mut split = rest.split_whitespace();
            let path = split.by_ref().next().expect("command should be present");
            let command = match path {
                "/" => Commands::Empty,
                echo if echo.starts_with("/echo/") => {
                    let res = echo
                        .splitn(3, "/")
                        .skip(2)
                        .flat_map(|s| s.chars())
                        .collect::<String>();
                    Commands::Echo(res)
                }
                agent if agent.starts_with("/user-agent") => {
                    let (user_agent, content) = user_agent
                        .split_once(": ")
                        .expect("user agent should delimit with :");
                    assert_eq!(user_agent, "User-Agent");
                    Commands::UserAgent(content.trim_end().to_string())
                }
                _ => Commands::Unknown
            };
            let version = split.next().expect("should contain HTTP version");
            assert_eq!(version, "HTTP/1.1");
            assert!(split.next().is_none());
            let header = Header {
                method,
                path: path.to_string(),
                command,
            };
            return Ok(Self {
                header,
                body: None,
            });
        } else {
            Err(anyhow!("invalid start line"))
        }
    }

    pub fn get_response(self) -> Response {
        match self.header.command {
            Commands::Empty => {
                Response {
                    status: HttpStatus::Ok,
                    content: None,
                }
            }
            Commands::Echo(body) => {
                let content = Some(Content {
                    content_type: "text/plain".to_string(),
                    content_length: body.len(),
                    body,
                });
                Response {
                    status: HttpStatus::Ok,
                    content,
                }
            }
            Commands::UserAgent(body) => {
                let content = Some(Content {
                    content_type: "text/plain".to_string(),
                    content_length: body.len(),
                    body,
                });
                Response {
                    status: HttpStatus::Ok,
                    content,
                }
            }
            Commands::Unknown => {
                Response {
                    status: HttpStatus::NotFound,
                    content: None,
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
}

impl std::fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match *self {
            HttpStatus::Ok => "HTTP/1.1 200 OK",
            HttpStatus::NotFound => "HTTP/1.1 404 NOT FOUND",
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    content_type: String,
    content_length: usize,
    body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    status: HttpStatus,
    content: Option<Content>,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.content {
            None => write!(f, "{}\r\n\r\n", self.status),
            Some(ref content) => {
                write!(f,
                       "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                       self.status,
                       content.content_type,
                       content.content_length,
                       content.body

                )
            }
        }
    }
}