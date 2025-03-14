use crate::engine::gamespace::{Distance, GSnum, Position};

use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};

impl Distance {
    pub fn new(p1: Position, p2: Position) -> Self {
        Self(p1, p2)
    }

    pub fn get_pos(&self) -> (Position, Position) {
        (self.0, self.1)
    }

    pub fn get_no_sqrt(&self) -> GSnum {
        let (Position(x1, y1), Position(x2, y2)) = self.get_pos();
        let dx = x2 - x1;
        let dy = y2 - y1;
        // dx^2 + dy^2
        dx * dx + dy * dy
    }

    pub fn get(&self) -> GSnum {
        // sqrt(dx^2 + dy^2)
        self.get_no_sqrt().sqrt()
    }

    fn cmp_pow2(&self, other: GSnum) -> Option<Ordering> {
        let dself = self.get_no_sqrt();
        if dself < other {
            Some(Less)
        } else if dself > other {
            Some(Greater)
        } else {
            Some(Equal)
        }
    }
}

impl PartialEq<Distance> for Distance {
    fn eq(&self, other: &Distance) -> bool {
        self.get_no_sqrt() == other.get_no_sqrt()
    }
}

impl PartialOrd<Distance> for Distance {
    fn partial_cmp(&self, other: &Distance) -> Option<Ordering> {
        let dother = other.get_no_sqrt();
        self.cmp_pow2(dother)
    }
}

impl PartialEq<GSnum> for Distance {
    fn eq(&self, other: &GSnum) -> bool {
        self.get() == *other
    }
}

impl PartialOrd<GSnum> for Distance {
    fn partial_cmp(&self, other: &GSnum) -> Option<std::cmp::Ordering> {
        // pow(other, 2) to avoid sqrt when cmp
        let dother = (*other) * (*other);
        self.cmp_pow2(dother)
    }
}
