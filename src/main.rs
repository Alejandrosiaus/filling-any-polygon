// main.rs
//
// Programa principal: crea una imagen de 800x600, define 5 poligonos
// (algunos convexos, algunos concavos), los rellena con el algoritmo
// de Scanline Fill (polygon.rs), dibuja sus bordes en blanco con el
// algoritmo de Bresenham (bresenham.rs) y exporta el resultado como
// out.png.
//
// El Poligono 5 se rellena con el color de fondo, de forma que quede
// visualmente "transparente" (se funde con el fondo de la imagen).

mod bresenham;
mod polygon;

use polygon::{draw_polygon, fill_polygon};
use raylib::prelude::*;

fn main() {
    let width: i32 = 800;
    let height: i32 = 600;

    // Color de fondo de la imagen.
    let background = Color::new(25, 25, 25, 255);

    // Se genera la imagen base rellena por completo con el color de fondo.
    // GenImageColor es una funcion de generacion de imagen (no es una
    // funcion de relleno de poligonos ni de dibujo de lineas).
    let mut image = Image::gen_image_color(width, height, background);

    // ---------------------------------------------------------------
    // Definicion de los poligonos (coordenadas dadas por el usuario).
    // ---------------------------------------------------------------

    // Poligono 1 (concavo, forma de estrella/flor).
    let poly1: Vec<(i32, i32)> = vec![
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ];

    // Poligono 2 (cuadrilatero).
    let poly2: Vec<(i32, i32)> = vec![(321, 335), (288, 286), (339, 251), (374, 302)];

    // Poligono 3 (triangulo).
    let poly3: Vec<(i32, i32)> = vec![(377, 249), (411, 197), (436, 249)];

    // Poligono 4 (concavo, forma de "corona").
    let poly4: Vec<(i32, i32)> = vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180),
    ];

    // Poligono 5 (cuadrilatero) - debe quedar "transparente" (color de fondo).
    let poly5: Vec<(i32, i32)> = vec![(682, 175), (708, 120), (735, 148), (739, 170)];

    // ---------------------------------------------------------------
    // Colores de relleno para cada poligono.
    // ---------------------------------------------------------------

    let color1 = Color::new(220, 60, 60, 255); // rojo
    let color2 = Color::new(60, 160, 220, 255); // azul
    let color3 = Color::new(240, 200, 60, 255); // amarillo
    let color4 = Color::new(90, 200, 120, 255); // verde
    let color5 = background; // "transparente": mismo color que el fondo

    let border_color = Color::WHITE;

    // ---------------------------------------------------------------
    // Relleno de cada poligono con Scanline Fill (manual).
    // ---------------------------------------------------------------

    fill_polygon(&mut image, &poly1, color1);
    fill_polygon(&mut image, &poly2, color2);
    fill_polygon(&mut image, &poly3, color3);
    fill_polygon(&mut image, &poly4, color4);
    fill_polygon(&mut image, &poly5, color5);

    // ---------------------------------------------------------------
    // Dibujo del borde de cada poligono con Bresenham (manual).
    // ---------------------------------------------------------------

    draw_polygon(&mut image, &poly1, border_color);
    draw_polygon(&mut image, &poly2, border_color);
    draw_polygon(&mut image, &poly3, border_color);
    draw_polygon(&mut image, &poly4, border_color);
    draw_polygon(&mut image, &poly5, border_color);

    // ---------------------------------------------------------------
    // Exportacion del resultado final.
    // ---------------------------------------------------------------

    image.export_image("out.png");
    println!("Imagen exportada como out.png");
}