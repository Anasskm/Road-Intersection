use sdl2::pixels::Color;

pub const OUTPUT_WIDTH: i32 = 800;
pub const OUTPUT_HEIGHT: i32 = 800;
pub const MIN_VELOCITY: i32 = 2;
pub const MAX_VELOCITY: i32 = 3;
pub const CAR_WIDTH: i32 = 20;
pub const CAR_HEIGHT: i32 = 20;
pub const SECURITY_DISTANCE: i32 = 30;
pub const CAR_COLOR_LEFT: Color = Color::RGB(255, 0, 0);
pub const CAR_COLOR_RIGHT: Color = Color::RGB(0, 255, 0);
pub const CAR_COLOR_STRAIGHT: Color = Color::RGB(0, 0, 255);
