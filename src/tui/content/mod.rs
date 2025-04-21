pub mod traits;
pub mod types_of_content;

use core::ops::{Deref, DerefMut};

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

impl DerefMut for Content {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
