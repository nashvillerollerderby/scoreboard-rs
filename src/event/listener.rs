use crate::event::ScoreBoardEvent;
use std::fmt::Display;

pub trait EventListener {}

pub trait ScoreBoardListener<T>: EventListener
where
    T: Clone + Display,
{
    fn score_board_change(&self, event: ScoreBoardEvent<T>);
}
