#![no_std]
// Triple buffer everything
pub enum GModes {}

pub type GCoord = usize;
pub type RGB = (u32, u32, u32);

pub const REFRESH_RATE: u8 = 60;

pub type Resolution = (usize, usize);
pub const DEFAULT_RESOLUTION: Resolution = (1440, 900);

pub type Point = (GCoord, GCoord);

fn put_pixel(coords: Point, color: RGB) {}

pub enum BuffPoint {
    Single,
    Double,
    Triple,
}

pub struct GraphicsBufferHandle {
    // Triple buffer??
    // current_buffer
    buffer_pointer: BuffPoint,
    current_buff: Buffer,
    double_buff: Buffer,
    triple_buff: Buffer,
}
pub type Buffer = [[RGB; 1440]; 900];

fn put_line(coords_start: Point, coords_end: Point, thickness: f32, color: RGB) {}
fn put_rect(coords_start: Point, coords_end: Point, color: RGB) {}
fn put_circle(coords: Point, radius: f32) {}
fn paint_cursor(coords: Point) {}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
