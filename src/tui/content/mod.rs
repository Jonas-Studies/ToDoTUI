pub struct Content {
    content: TypesOfContent
}

impl Content {
    pub fn new(content: TypesOfContent) -> Self {
        Self { content }
    }
}

pub enum TypesOfContent {
}
