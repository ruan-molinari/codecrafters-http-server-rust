pub enum Status {
    OK,
    NotFound,
    InternalServerError,
    BadRequest,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::OK => "200 OK",
            Status::NotFound => "404 Not Found",
            Status::InternalServerError => "500 Internal Server Error",
            Status::BadRequest => "400 Bad Request",
        }
    }
}
