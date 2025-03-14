use crate::engine::gamespace::{position::HasPosition, size::HasSize, GSnum, Position, Size};

/// HasGameRect provides two methods that need to be implemented:
/// get_center and set_center
///
/// The "center" of the GameRect is intended to be a relative value
/// where 0 is the "left" of the size, and 1 is the "right". It's the
/// same for the top and bottom.
pub trait HasGameRect: HasSize + HasPosition {
    fn get_center(&self) -> Position;
    fn set_center(&self, center: &Position);

    fn left(&self) -> GSnum {
        let (px, cx, sx) = self.get_pos_center_size_x();
        px - (cx * sx)
    }

    fn right(&self) -> GSnum {
        let (px, cx, sx) = self.get_pos_center_size_x();
        px + ((1.0 - cx) * sx)
    }

    fn bottom(&self) -> GSnum {
        let (py, cy, sy) = self.get_pos_center_size_y();
        py - (cy * sy)
    }

    fn top(&self) -> GSnum {
        let (py, cy, sy) = self.get_pos_center_size_y();
        py + ((1.0 - cy) * sy)
    }

    fn range_x(&self) -> (GSnum, GSnum) {
        (self.left(), self.right())
    }

    fn range_y(&self) -> (GSnum, GSnum) {
        (self.bottom(), self.top())
    }

    fn get_pos_center_size_x(&self) -> (GSnum, GSnum, GSnum) {
        let Position(px, _) = self.get_pos();
        let Position(cx, _) = self.get_center();
        let Size(sx, _) = self.get_size();
        (px, cx, sx)
    }

    fn get_pos_center_size_y(&self) -> (GSnum, GSnum, GSnum) {
        let Position(_, py) = self.get_pos();
        let Position(_, cy) = self.get_center();
        let Size(_, sy) = self.get_size();
        (py, cy, sy)
    }

    fn intersects<G: HasGameRect + ?Sized>(&self, _other: &G) -> bool {
        true
    }
}
