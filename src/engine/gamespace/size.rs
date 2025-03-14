use super::Size;

pub trait HasSize {
    fn get_size(&self) -> Size;
    fn set_size(&self, size: &Size);
}
