#![no_std]

pub enum GModes {
    Vga800x600,
    Custom(u16, u16),
}

pub type GCoord = usize;
pub type RGB = (u32, u32, u32);
pub type RefreshRate = u8;
pub const REFRESH_RATE: u8 = 60;

pub type Resolution = (usize, usize);

pub type Point = (GCoord, GCoord);

pub struct FrameBuffer;

pub trait Graphics {
    fn put_line(&self, coords_start: Point, coords_end: Point, thickness: u32, color: RGB);
    fn put_rect(&self, coords_start: Point, coords_end: Point, color: RGB);
    fn put_circle(&self, coords: Point, radius: u32);
    fn put_pixel(&self, coords: Point, color: RGB);

    fn paint_cursor(&self, coords: Point);
    fn hide_cursor(&self);
    fn show_cursor(&self);
    /// Actually move the double buffer to the single buffer and "update" the screen
    fn draw(&self);
    fn clear(&self);
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
