// polygon.rs
//
// Contiene las funciones para dibujar el borde de un poligono
// (usando el algoritmo de Bresenham de bresenham.rs) y para rellenarlo
// mediante el algoritmo de Scanline Fill, implementado manualmente
// pixel por pixel sobre una Image de Raylib.
//
// NOTA: No se utiliza ninguna funcion de relleno ni de dibujo de lineas
// de Raylib (no draw_line, no ImageDrawLine, no ImageFill, etc.).
// El unico metodo de Raylib empleado es image.draw_pixel(), que pinta
// un unico pixel dentro de la imagen; toda la geometria (lineas y
// relleno) se calcula manualmente en este archivo y en bresenham.rs.

use crate::bresenham::bresenham_line;
use raylib::prelude::*;

/// Dibuja el borde de un poligono definido por una lista de vertices
/// (en orden, ya sea horario o antihorario), conectando cada vertice
/// con el siguiente y cerrando la figura (ultimo vertice -> primero).
///
/// El trazado de cada arista se realiza con el algoritmo de Bresenham
/// implementado manualmente en bresenham.rs.
pub fn draw_polygon(image: &mut Image, vertices: &[(i32, i32)], color: Color) {
    let n = vertices.len();
    if n < 2 {
        return;
    }

    for i in 0..n {
        let (x0, y0) = vertices[i];
        let (x1, y1) = vertices[(i + 1) % n];

        let points = bresenham_line(x0, y0, x1, y1);

        for (x, y) in points {
            image.draw_pixel(x, y, color);
        }
    }
}

/// Rellena un poligono (convexo o concavo, con cualquier cantidad de
/// vertices) utilizando el algoritmo de Scanline Fill:
///
/// 1. Para cada linea horizontal (scanline) que cruza el poligono, se
///    calculan las intersecciones de esa linea con todas las aristas.
/// 2. Las intersecciones se ordenan de menor a mayor en X.
/// 3. Se rellenan los pixeles entre cada par consecutivo de
///    intersecciones (par 0-1, par 2-3, par 4-5, ...).
///
/// Las aristas horizontales se ignoran (no aportan interseccion util),
/// y se usa el intervalo semiabierto [y_min, y_max) en cada arista para
/// evitar contar dos veces los vertices donde coinciden dos aristas.
pub fn fill_polygon(image: &mut Image, vertices: &[(i32, i32)], color: Color) {
    let n = vertices.len();
    if n < 3 {
        return;
    }

    // Rango vertical del poligono.
    let y_min = vertices.iter().map(|v| v.1).min().unwrap();
    let y_max = vertices.iter().map(|v| v.1).max().unwrap();

    for y in y_min..=y_max {
        let mut intersections: Vec<f64> = Vec::new();

        // Se recorren todas las aristas del poligono.
        for i in 0..n {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[(i + 1) % n];

            // Se ignoran las aristas horizontales.
            if y1 == y2 {
                continue;
            }

            let (y_lo, y_hi) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

            // Intervalo semiabierto [y_lo, y_hi) para evitar duplicar
            // intersecciones exactamente en los vertices.
            if y >= y_lo && y < y_hi {
                let t = (y - y1) as f64 / (y2 - y1) as f64;
                let x = x1 as f64 + t * (x2 - x1) as f64;
                intersections.push(x);
            }
        }

        // Se ordenan las intersecciones de izquierda a derecha.
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Se rellena entre cada par consecutivo de intersecciones.
        let mut i = 0;
        while i + 1 < intersections.len() {
            let x_start = intersections[i].round() as i32;
            let x_end = intersections[i + 1].round() as i32;

            for x in x_start..=x_end {
                image.draw_pixel(x, y, color);
            }

            i += 2;
        }
    }
}