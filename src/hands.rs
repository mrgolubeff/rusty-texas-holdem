pub enum Hands {
    RoyalFlush(u16),
    StraightFlush(u16),
    FourOfAKind(u16),
    FullHouse(u16),
    Flush(u16),
    Straight(u16),
    ThreeOfAKind(u16),
    TwoPair(u16),
    Pair(u16),
    HighCard(u16),
}
