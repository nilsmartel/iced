use crate::{Bus, Element, Widget};
use dodrio::bumpalo;

pub use iced_core::text::*;

impl<'a, Message> Widget<Message> for Text {
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        _publish: &Bus<Message>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        let content = bumpalo::format!(in bump, "{}", self.content);
        let size = bumpalo::format!(in bump, "font-size: {}px", self.size.unwrap_or(20));

        // TODO: Complete styling
        p(bump)
            .attr("style", size.into_bump_str())
            .children(vec![text(content.into_bump_str())])
            .finish()
    }
}

impl<'a, Message> From<Text> for Element<'a, Message> {
    fn from(text: Text) -> Element<'a, Message> {
        Element::new(text)
    }
}
