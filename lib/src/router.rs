use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, handler: &str) {
        self.routes.insert(path.to_string(), handler.to_string());
    }

    pub fn handle_request(&self, path: &str) -> Option<&String> {
        self.routes.get(path)
    }
}
