use rand::Rng;

use crate::util::point::Point;


#[derive(Debug)]
pub struct FaultZoneStrip<'this> {
    pub input_domain: &'this [Vec<i32>],
    pub edge: i32,
    pub above_line_delta: f64,
    pub below_line_delta: f64,
    pub ratio: f64,
    pub theta: f64,
}

impl<'this> FaultZoneStrip<'this> {
    pub fn find_target(&self, p: &Point) -> bool {
        (p.coordinates[1] - self.ratio as f32 * p.coordinates[0] >= self.below_line_delta as f32)
            && (p.coordinates[1] - self.ratio as f32 * p.coordinates[0] <= self.above_line_delta as f32)
    }

    pub fn get_theta(&self) -> f64 {
        self.theta
    }

    pub fn new(boundary: &'this [Vec<i32>], area: f64, rate: f64) -> FaultZoneStrip<'this> {
        let mut fzs = Self {
            input_domain: boundary,
            edge: boundary[0][1] - boundary[0][0],
            above_line_delta: 0.0,
            below_line_delta: 0.0,
            ratio: 0.0,
            theta: area,
        };

        let mut rng = rand::rng();
        let line_location = rng.random_range(0..3);

        let (mut p1x, mut p1y, mut p2x, mut p2y, mut p3x, mut p3y, mut p4x, mut p4y) =
            (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        if line_location == 0 {
            loop {
                p1x = -5000.0;
                p2x = -5000.0;
                p2y = -5000.0 + (10000.0 * rate * rng.random::<f64>());
                p3y = 5000.0;
                p4x = (-5000.0 + (10000.0 * (1.0 - rate))) + (10000.0 * rate * rng.random::<f64>());
                p4y = 5000.0;

                let big_triangle_area = (5000.0 - p2y) * (p4x + 5000.0) / 2.0;
                fzs.ratio = (p4y - p2y) / (p4x - p2x);
                let temp = 2.0 * (big_triangle_area - 10000.0 * 10000.0 * area) / fzs.ratio;
                p3x = temp.sqrt() - 5000.0;
                p1y = 5000.0 - fzs.ratio * (p3x + 5000.0);

                if (p3x >= (-5000.0 + (10000.0 * (1.0 - rate))))
                    && (p1y <= (-5000.0 + 10000.0 * rate))
                {
                    break;
                }
            }
        } else if line_location == 1 {
            loop {
                p1x = -5000.0;
                p2x = -5000.0;
                p2y = -5000.0 + (10000.0 * rng.random::<f64>());
                p3x = 5000.0;
                p4x = 5000.0;
                p4y = -5000.0 + (10000.0 * rng.random::<f64>());
                p1y = p2y + 10000.0 * area;
                p3y = p4y + 10000.0 * area;
                fzs.ratio = (p4y - p2y) / (p4x - p2x);
                if p1y <= 5000.0 && p3y <= 5000.0 {
                    break;
                }
            }
        } else {
            loop {
                p1x = -5000.0;
                p1y = (-5000.0 + (10000.0 * (1.0 - rate))) + (10000.0 * rate * rng.random::<f64>());
                p2x = -5000.0;
                p3x = (-5000.0 + (10000.0 * (1.0 - rate))) + (10000.0 * rate * rng.random::<f64>());
                p3y = -5000.0;
                p4y = -5000.0;

                fzs.ratio = (p3y - p1y) / (p3x - p1x);
                let big_triangle_area = (p1y + 5000.0) * (p3x + 5000.0) / 2.0;
                let temp = 2.0 * (10000.0 * 10000.0 * area - big_triangle_area) / fzs.ratio;
                p4x = temp.sqrt() - 5000.0;
                p2y = -fzs.ratio * (p4x + 5000.0) - 5000.0;

                if (p4x >= (-5000.0 + (10000.0 * (1.0 - rate))))
                    && (p2y >= (-5000.0 + (10000.0 * (1.0 - rate))))
                {
                    break;
                }
            }
        }

        fzs.above_line_delta = p1y - fzs.ratio * p1x;
        fzs.below_line_delta = p4y - fzs.ratio * p4x;
        fzs
    }
}
