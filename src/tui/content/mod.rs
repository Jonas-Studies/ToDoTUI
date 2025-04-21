pub mod traits;
pub mod types_of_content;

use core::ops::{Deref, DerefMut};

use types_of_content::TypesOfContent;

pub struct Content {
    content: TypesOfContent,
    can_be_focused: bool
}

impl Content {
    pub fn new(content: TypesOfContent) -> Self {
        Self { content, can_be_focused: false }
    }
    pub fn as_can_be_focused(mut self) -> Self {
        self.can_be_focused = true;
        self
    }
    pub fn can_be_focused(&self) -> bool {
        self.can_be_focused.clone()
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
