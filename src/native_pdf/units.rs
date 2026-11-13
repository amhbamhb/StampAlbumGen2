pub fn mm_to_points(mm: f64) -> f64 {
    mm * (72.0 / 25.4)
}

pub fn points_to_mm(points: f64) -> f64 {
    points * (25.4 / 72.0)
}
