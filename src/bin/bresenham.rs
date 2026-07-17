use raylib::prelude::*;

fn bresenham(
    image: &mut Image,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    loop {

        // Dibujar píxel
        image.draw_pixel(x0, y0, color);

        // Llegamos al final
        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x0 += sx;
        }

        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn main() {

    let mut image = Image::gen_image_color(
        500,
        500,
        Color::BLACK,
    );

    bresenham(
        &mut image,
        50,
        50,
        450,
        300,
        Color::WHITE,
    );

    image.export_image("linea_bresenham.png");

    println!("Línea Bresenham creada.");
}