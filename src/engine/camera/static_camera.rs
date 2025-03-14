use super::Camera;

use_gamespace!();

struct StaticCamera {
    pos: Position,
    center: Position,
    size: Size,

    dst: Size,
}

impl Camera for StaticCamera {
    fn get_dst_size(&self) -> Size {
        self.dst
    }
}

impl HasPosition for StaticCamera {
    fn get_pos(&self) -> Position {
        self.pos
    }
    fn set_pos(&self, _pos: &Position) {

    }
}

impl HasSize for StaticCamera {
    fn get_size(&self) -> Size {
        self.size
    }
    fn set_size(&self, _size: &Size) {
        
    }
}

impl HasGameRect for StaticCamera {
    fn get_center(&self) -> Position {
        self.center
    }

    fn set_center(&self, _center: &Position) {

    }
}