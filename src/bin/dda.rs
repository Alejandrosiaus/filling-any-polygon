use raylib::prelude::*;

fn dda(
    image: &mut Image,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;

    let pasos = dx.abs().max(dy.abs());

    let x_inc = dx as f32 / pasos as f32;
    let y_inc = dy as f32 / pasos as f32;

    let mut x = x0 as f32;
    let mut y = y0 as f32;

    for _ in 0..=pasos {
        image.draw_pixel(
            x.round() as i32,
            y.round() as i32,
            color,
        );

        x += x_inc;
        y += y_inc;
    }
}

fn main() {
    let mut image = Image::gen_image_color(
        500,
        500,
        Color::BLACK,
    );

    // Dibujar una línea
    dda(
        &mut image,
        50,
        50,
        450,
        300,
        Color::WHITE,
    );

    image.export_image("linea_dda.png");

    println!("Línea DDA creada.");
}