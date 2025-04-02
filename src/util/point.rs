use rand::Rng;


#[derive(Debug, Clone)]
pub struct Point {
    pub coordinates: Vec<f32>,
    pub n: usize,
}

impl Point {
    pub fn new(n: usize) -> Self {
        Self {
            coordinates: vec![0.0; n],
            n,
        }
    }

    pub fn generate_rand_p(bound: &[Vec<i32>]) -> Self {
        let n = bound.len();
        let mut p = Point::new(n);
        let mut rng = rand::rng();
        (0..n).for_each(|i| {
            p.coordinates[i] = (bound[i][0] as f32) + ((bound[i][1] - bound[i][0]) as f64 * rng.random::<f64>()) as f32;
            // p.coordinates[i] = rng.random_range(bound[i][0] as f32..bound[i][1] as f32);
        });
        p
    }

    pub fn get_distance(p1: &Self, p2: &Self) -> f32 {
        let mut sum_sq = 0.0;
        for i in 0..p1.n {
            sum_sq += (p1.coordinates[i] - p2.coordinates[i]).powi(2);
        }
        sum_sq.sqrt()
    }
}

