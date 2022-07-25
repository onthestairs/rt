use crate::colour::Colour;
use rayon::prelude::*;

pub fn print_image(width: u64, height: u64, image: Vec<Vec<Colour>>) {
    print!("P3\n{width} {height}\n255\n");
    image.into_iter().rev().for_each(|row| {
        row.into_iter().for_each(|colour| print_colour(&colour));
    })
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    return x;
}

fn print_colour(colour: &Colour) {
    let red = (clamp(colour.red, 0.0, 0.999) * 256.0) as u64;
    let green = (clamp(colour.green, 0.0, 0.999) * 256.0) as u64;
    let blue = (clamp(colour.blue, 0.0, 0.999) * 256.0) as u64;
    println!("{red} {green} {blue}");
}

pub fn generate_gradient(width: u64, height: u64) -> Vec<Vec<Colour>> {
    return generate_image(width, height, |row, col| {
        return Colour {
            red: col as f64 / (width - 1) as f64,
            green: row as f64 / (height - 1) as f64,
            blue: 0.25,
        };
    });
}

pub fn generate_image<F>(width: u64, height: u64, mut f: F) -> Vec<Vec<Colour>>
where
    F: FnMut(u64, u64) -> Colour,
{
    return (0..height)
        .par_iter()
        .map(|row| {
            return (0..width)
                .into_iter()
                .map(|col| {
                    return f(row, col);
                })
                .collect();
        })
        .collect();
}
