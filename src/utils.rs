pub fn scale(proportion: f64, min: f64, max: f64) -> f64 {
    return min + proportion * (max - min);
}
