fn magnitude(vector: &[f64]) -> f64 {
    let mut sum_of_squares = 0.0;
    for &coordinate in vector {
        sum_of_squares += coordinate * coordinate;
    }
    sum_of_squares.sqrt()
}

fn normalize(vector: &mut [f64]) {
    let mag = magnitude(vector);

    for coordinate in vector {
        *coordinate /= mag;
    }
}

fn main() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
