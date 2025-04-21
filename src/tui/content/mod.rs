pub mod traits;
pub mod types_of_content;

use core::ops::Deref;

use types_of_content::TypesOfContent;

pub struct Content {
    content: TypesOfContent
}

impl Content {
    pub fn new(content: TypesOfContent) -> Self {
        Self { content }
    }
}

impl Deref for Content {
    type Target = TypesOfContent;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
