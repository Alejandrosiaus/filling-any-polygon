// bresenham.rs
//
// Contiene UNICAMENTE la implementacion manual del algoritmo de Bresenham
// para el trazado de lineas rectas entre dos puntos enteros.
//
// No depende de Raylib ni de ninguna otra libreria: recibe coordenadas
// enteras y devuelve la lista de puntos (pixeles) que forman la linea.

/// Calcula todos los puntos (x, y) que forman la linea entre (x0, y0) y
/// (x1, y1) utilizando el algoritmo de Bresenham para lineas enteras.
///
/// Esta implementacion soporta los 8 octantes (cualquier direccion:
/// izquierda-derecha, arriba-abajo, diagonales de cualquier pendiente).
pub fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        points.push((x, y));

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x += sx;
        }

        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }

    points
}