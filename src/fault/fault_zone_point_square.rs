use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::util::point::Point;

#[derive(Debug)]
pub struct FaultZonePointSquare<'this> {
    pub input_domain: &'this [Vec<i32>],
    pub n_points: usize,
    pub fault_points: Vec<Point>,
    pub delta: f64,
    pub theta: f64,
}

impl<'this> FaultZonePointSquare<'this> {
    pub fn new(input_domain: &'this [Vec<i32>], theta: f64) -> Self {
        let n_dims = input_domain.len();
        let n_points = 25;

        // println!("theta: {theta}");

        let mut sum = 1.0f64;
        (0..n_dims).for_each(|i| {
            sum *= (input_domain[i][1] - input_domain[i][0]) as f64;
        });
        let delta = (sum * theta / n_points as f64).powf(1.0 / n_dims as f64);

        let mut fault_points = vec![];

        let mut n_overlaps = 0;
        let mut rng = StdRng::from_os_rng();

        while fault_points.len() < n_points {
            let mut fault_point_candidate = Point::new(n_dims);
            loop {
                (0..n_dims).for_each(|i| {
                    let coordinate = input_domain[i][0] as f32
                        + (((input_domain[i][1] - input_domain[i][0]) as f64 - delta)
                            * rng.random::<f64>()) as f32;
                    fault_point_candidate.coordinates[i] = coordinate;
                });

                if !Self::is_overlap(&fault_point_candidate, delta, &fault_points) {
                    break;
                }
                n_overlaps += 1;
                if n_overlaps > 1e6 as usize {
                    fault_points.clear();
                    n_overlaps = 0;
                }
            }
            fault_points.push(fault_point_candidate);

            // Print maximum and minimum values for each dimension across all candidates
            // let mut max = vec![0.0; n_dims];
            // let mut min = vec![0.0; n_dims];
            // for i in 0..n_dims {
            //     max[i] = fault_points.iter().map(|p| p.coordinates[i]).fold(f32::MIN, f32::max);
            //     min[i] = fault_points.iter().map(|p| p.coordinates[i]).fold(f32::MAX, f32::min);
            // }
            // println!("max: {:?}", max);
            // println!("min: {:?}", min);

        }

        Self {
            input_domain,
            n_points,
            fault_points,
            delta,
            theta,
        }
    }

    fn is_overlap(p: &Point, delta: f64, fault_points: &[Point]) -> bool {
        if fault_points.is_empty() {
            return false;
        }

        for fault_point in fault_points {
            let mut overlap = true;
            for dim in 0..p.n {
                if (p.coordinates[dim] - fault_point.coordinates[dim]).abs() as f64 > delta {
                    overlap = false;
                    break;
                }
            }
            if overlap {
                return true;
            }
        }
        false
    }

    pub fn find_target(&self, p: &Point) -> bool {
        for fault_point in &self.fault_points {
            let mut found = true;
            for j in 0..p.n {
                if !(p.coordinates[j] >= fault_point.coordinates[j]
                    && p.coordinates[j] as f64 <= fault_point.coordinates[j] as f64 + self.delta)
                {
                    found = false;
                    break;
                }
            }
            if found {
                return true;
            }
        }
        false
    }

    pub fn get_theta(&self) -> f64 {
        self.theta
    }
}
