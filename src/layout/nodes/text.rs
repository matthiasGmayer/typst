use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

use fontdock::{FallbackTree, FontVariant};

use super::*;
use crate::shaping;

/// A text node.
#[derive(Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub size: f64,
    pub dir: Dir,
    pub fallback: Rc<FallbackTree>,
    pub variant: FontVariant,
    pub aligns: Gen2<GenAlign>,
}

#[async_trait(?Send)]
impl Layout for Text {
    async fn layout(
        &self,
        ctx: &mut LayoutContext,
        _constraints: LayoutConstraints,
    ) -> Vec<LayoutItem> {
        let mut loader = ctx.loader.borrow_mut();
        let boxed = shaping::shape(
            &self.text,
            self.size,
            self.dir,
            &mut loader,
            &self.fallback,
            self.variant,
        )
        .await;
        vec![LayoutItem::Box(boxed, self.aligns)]
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Text({})", self.text)
    }
}

impl From<Text> for LayoutNode {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}