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
    pub buffer_pointer: BuffPoint,
    pub current_buff: Buffer,
    pub double_buff: Buffer,
    pub triple_buff: Buffer,
}
pub type Buffer = [[RGB; 1440]; 900];

fn put_line(coords_start: Point, coords_end: Point, thickness: f32, color: RGB) {}
fn put_rect(coords_start: Point, coords_end: Point, color: RGB) {}
fn put_circle(coords: Point, radius: f32) {}
fn paint_cursor(coords: Point) {}

pub fn prelim_testing() {
    let vga_buffer = 0xb8000 as *mut u8;
    let hello: &[u8] = b"Running on x86_64";

    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
