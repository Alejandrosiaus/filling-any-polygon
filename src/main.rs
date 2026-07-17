use raylib::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// Escribe un píxel en el buffer
fn plot(buffer: &mut Vec<Color>, x: i32, y: i32, color: Color) {
    if x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT {
        let index = (y * WIDTH + x) as usize;
        buffer[index] = color;
    }
}

// Algoritmo usando la ecuación de la recta
fn linea_ecuacion(
    buffer: &mut Vec<Color>,
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    color: Color,
) {
    // Línea vertical
    if x0 == x1 {
        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
        }

        for y in y0..=y1 {
            plot(buffer, x0, y, color);
        }

        return;
    }

    // Ordenar por X
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let m = (y1 - y0) as f32 / (x1 - x0) as f32;
    let b = y0 as f32 - m * x0 as f32;

    for x in x0..=x1 {
        let y = (m * x as f32 + b).round() as i32;
        plot(buffer, x, y, color);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Linea por ecuacion")
        .build();

    rl.set_target_fps(60);

    // Buffer negro
    let mut buffer = vec![Color::BLACK; (WIDTH * HEIGHT) as usize];

    // Dibujar una línea en el buffer
    linea_ecuacion(
        &mut buffer,
        100,
        100,
        700,
        400,
        Color::RED,
    );

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        // Dibujar todo el buffer
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = (y * WIDTH + x) as usize;
                d.draw_pixel(x, y, buffer[index]);
            }
        }
    }
}