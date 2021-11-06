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

pub trait GraphicsBuffer {
    fn put_line(coords_start: Point, coords_end: Point, thickness: f32, color: RGB);
    fn put_rect(coords_start: Point, coords_end: Point, color: RGB);
    fn put_circle(coords: Point, radius: f32);
    fn put_pixel(coords: Point, color: RGB);

    fn paint_cursor(coords: Point);
    fn hide_cursor();
    fn show_cursor();

    fn draw();
    fn clear();
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
