#[derive(Debug)]
pub struct CardTemplate {}

const Forest: CardTemplate = CardTemplate {};

pub type Card = &'static CardTemplate;