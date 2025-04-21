pub mod traits;
pub mod types_of_content;

use types_of_content::TypesOfContent;

pub struct Content {
    content: TypesOfContent
}

impl Content {
    pub fn new(content: TypesOfContent) -> Self {
        Self { content }
    }
}
