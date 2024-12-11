use piston_window::{rectangle, types::Color, Context, G2d};
use crate::game::BLOCK_SIZE;

pub fn to_gui_coord(coord: f64) -> f64 {
    coord * BLOCK_SIZE
}

pub fn _draw_rectangle(color: Color, x: f64, y: f64, height: f64, width: f64, ctx: &Context, g: &mut G2d) {
    rectangle(color, [to_gui_coord(x), to_gui_coord(y), BLOCK_SIZE as f64 * width, BLOCK_SIZE as f64 * height], ctx.transform, g);
}

#[macro_export]
macro_rules! draw_rectangle {
    // use provided heigth and width
    ($color:expr, $x:expr, $y:expr, $height:expr, $width:expr, $ctx:expr, $g:expr) => {
        crate::draw::_draw_rectangle($color, $x, $y, $height, $width, $ctx, $g)
    };

    // use default heigth and width
    ($color:expr, $x:expr, $y:expr, $ctx:expr, $g:expr) => {
        crate::draw::_draw_rectangle($color, $x, $y, 1.0, 1.0, $ctx, $g)
    };
}
