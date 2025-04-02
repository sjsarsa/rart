use crate::util::point::Point;
use rand::Rng;

#[derive(Debug)]
pub struct FaultZoneBlock<'a> {
    pub input_domain: &'a [Vec<i32>],
    pub fault_point: Point,
    pub delta: f64,
    pub theta: f64,
}

impl<'a> FaultZoneBlock<'a> {
    pub fn new(boundary: &'a [Vec<i32>], area: f64) -> Self {
        let n = boundary.len();
        let mut sum = 1.0;
        (0..n).for_each(|i| {
            sum *= (boundary[i][1] - boundary[i][0]) as f64;
        });
        let delta = (sum * area).powf(1.0 / n as f64);

        let mut fault_point = Point::new(n);
        (0..n).for_each(|i| {
            fault_point.coordinates[i] = boundary[i][0] as f32
                + ((boundary[i][1] - boundary[i][0]) as f32 - delta as f32) * rand::rng().random::<f32>();
        });

        Self {
            input_domain: boundary,
            fault_point,
            delta,
            theta: area,
        }
    }

    pub fn find_target(&self, p: &Point) -> bool {
        for i in 0..p.n {
            if !((p.coordinates[i] >= self.fault_point.coordinates[i])
                && (p.coordinates[i] <= self.fault_point.coordinates[i] + self.delta as f32))
            {
                return false;
            }
        }
        true
    }

    pub fn get_theta(&self) -> f64 {
        self.theta
    }
}
