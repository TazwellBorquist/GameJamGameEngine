pub mod distance;
pub mod gamerect;
pub mod position;
pub mod size;

pub type GSnum = f32;

#[derive(Clone, Copy, Debug)]
pub struct Size(pub GSnum, pub GSnum);

#[derive(Clone, Copy, Debug)]
pub struct Position(pub GSnum, pub GSnum);

#[derive(Clone, Copy, Debug)]
pub struct Distance(pub Position, pub Position);

#[macro_export]
macro_rules! use_gamespace {
    () => {
        #[allow(unused_imports)]
        use crate::engine::gamespace::{
            gamerect::HasGameRect, position::HasPosition, size::HasSize, Distance, GSnum, Position,
            Size,
        };
    };
}
