pub struct CardTemplate {}

const Forest: CardTemplate = CardTemplate {};

pub(crate) type Card = &'static CardTemplate;