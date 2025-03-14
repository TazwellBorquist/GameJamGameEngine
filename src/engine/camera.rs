pub mod static_camera;

use sdl2::rect::Rect;

use_gamespace!();

pub trait Camera: HasGameRect {
    fn get_dst_size(&self) -> Size;

    fn dst_rect_of<O: HasGameRect + ?Sized>(&self, obj: &O) -> Option<Rect> {
        if self.intersects(obj) {
            let src_size = self.get_size();
            let dst_size = self.get_dst_size();

            let dx = obj.left() - self.left();
            let dy = obj.top() - self.top();

            let Size(sx, sy) = obj.get_size();

            let rx = dst_size.0 / src_size.0;
            let ry = dst_size.1 / src_size.1;

            let pdx = (dx * rx).floor() as i32;
            let pdy = (dy * ry).floor() as i32;
            let psx = (sx * rx).round() as u32;
            let psy = (sy * ry).round() as u32;

            Some(
                Rect::new(
                    pdx,
                    pdy,
                    psx,
                    psy
                )
            )
        } else {
            None
        }
    }
}