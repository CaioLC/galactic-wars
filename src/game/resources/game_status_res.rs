pub struct GameStatus(pub GameStatusEnum);
pub enum GameStatusEnum {
    Uninitialized,
    Started(u64),
    Finished,
}
