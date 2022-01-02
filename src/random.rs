use rand::Rng;

pub fn randf() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn bounded_randf(min: f64, max: f64) -> f64 {
    min + (max - min)  * randf()
}