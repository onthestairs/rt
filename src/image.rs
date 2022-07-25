use crate::colour::Colour;

pub fn print_image(width: u64, height: u64, image: Vec<Vec<Colour>>) {
    print!("P3\n{width} {height}\n255\n");
    image.into_iter().rev().for_each(|row| {
        row.into_iter().for_each(|colour| print_colour(&colour));
    })
}

fn print_colour(colour: &Colour) {
    let red = (colour.red * 255.0) as u64;
    let green = (colour.green * 255.0) as u64;
    let blue = (colour.blue * 255.0) as u64;
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
        .into_iter()
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
