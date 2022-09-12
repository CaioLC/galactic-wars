use crate::game::components::characteristics::DestinationEnum;

pub struct GameStatus(pub GameStatusEnum);
pub enum GameStatusEnum {
    Uninitialized,
    Started(u64),
    Finished,
}

pub struct IsTradeRouting {
    pub key_down: bool,
    pub trade_route: Vec<DestinationEnum>,
}
