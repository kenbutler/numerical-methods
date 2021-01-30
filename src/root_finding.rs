use log::{info, error, debug};

pub struct Data {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

pub fn bisection(data: Vec<f32>) {

}

pub fn newton_raphson(data: &Data) {
    let mut lower_bound = data.x.iter().cloned().fold(0./0., f32::min);
    let mut upper_bound = data.x.iter().cloned().fold(0./0., f32::max);
    info!("{}, {}", lower_bound, upper_bound);
}

/// Unit tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_newton_raphson() {
        let mut x_data: Vec<f32> = vec![];
        let mut y_data: Vec<f32> = vec![];
        for i in -10..11 {
            let x = i as f32;
            x_data.push(x);
            y_data.push(x * x);
        }
        let data = Data { x: x_data, y: y_data };
        newton_raphson(&data);
    }
}

