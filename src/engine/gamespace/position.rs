use crate::engine::gamespace::{Distance, GSnum, Position};

pub trait HasPosition {
    fn get_pos(&self) -> Position;
    fn set_pos(&self, pos: &Position);

    fn offset(&self, x: f32, y: f32) {
        let Position(xs, ys) = self.get_pos();
        let newpos = Position(xs + x, ys + y);
        self.set_pos(&newpos);
    }

    fn dist_to<P: HasPosition + ?Sized>(&self, other: &P) -> Distance {
        Distance::new(self.get_pos(), other.get_pos())
    }
}

impl HasPosition for (GSnum, GSnum) {
    fn get_pos(&self) -> Position {
        Position(self.0, self.1)
    }

    fn set_pos(&self, _pos: &Position) {
        ()
    }
}
