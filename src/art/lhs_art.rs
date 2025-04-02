use crate::{fault::fault_zone::FaultZone, util::point::Point};
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct LhsArt<'this> {
    /// Number of partitions per domain dimension.
    /// The total number of hypercubes is n_partitions^n,
    /// where n is the number of dimensions in the
    pub n_partitions: usize,
    pub input_domain: &'this [Vec<i32>],
    pub exhaustive: bool,
    randomised_point_indices: Vec<u32>,
}

impl Default for LhsArt<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'this> LhsArt<'this> {
    pub fn new() -> LhsArt<'this> {
        LhsArt {
            n_partitions: 10,
            input_domain: &[],
            exhaustive: false,
            randomised_point_indices: vec![],
        }
    }

    pub fn with_partition_count(n_partitions: usize) -> Self {
        Self {
            n_partitions,
            input_domain: &[],
            exhaustive: false,
            randomised_point_indices: vec![],
        }
    }

    /// Populates a test case suite with random points using latin hypercube sampling
    ///
    /// From https://www.statisticshowto.com/latin-hypercube-sampling/:
    /// One-dimensional Latin hypercube sampling involves dividing your cumulative density function (cdf) into n equal partitions; and then choosing a random data point in each partition.
    ///
    /// As a simple example, let’s say you needed a random sample with 100 data points. First, divide the cdf into 100 equal intervals. If your distribution starts at 0 and ends with k , your first data point would be selected from the interval between (0,k/100). The second data point would be from the interval (k/100, 2k/100), your third from (2k/100, 3k/100), and so on. In each interval you would randomly select one point, giving you 100 different points.
    ///
    /// Two-dimensional Latin hypercube sampling is not much more complicated and is usually performed with software. Assuming your two variables, x1 and x2 are independent, you follow the one-dimensional method to come up with one dimensional samples for x1 and x2 separately. Once you have two lists of samples, you combine them, randomly, into two-dimensional pairs.
    ///
    /// For n-dimensional Latin hypercube sampling the same method is used.
    fn populate_test_cases_random(&self, existing_test_cases: &mut Vec<Point>) {
        let n = self.input_domain.len();
        let mut points = Vec::with_capacity(self.n_partitions);

        let ranges = (0..n)
            .map(|i| self.input_domain[i][0]..self.input_domain[i][1])
            .collect::<Vec<_>>();
        let steps = self.compute_steps();

        let mut lower_bounds = ranges
            .iter()
            .zip(steps.iter())
            .map(|(r, s)| {
                r.clone()
                    .step_by(*s as usize)
                    .map(|bound| bound as f64)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        (0..self.n_partitions).for_each(|_| {
            let point = Point {
                coordinates: (0..n)
                    .map(|d| {
                        let pick_index = rand::random_range(0..lower_bounds[d].len());
                        let lower_bound = lower_bounds[d].remove(pick_index);

                        let upper_bound =
                            f32::min(lower_bound as f32 + steps[d] as f32, ranges[d].end as f32);
                        rand::random_range(lower_bound as f32..upper_bound)
                    })
                    .collect(),
                n,
            };
            points.push(point);
        });

        existing_test_cases.append(&mut points);
    }

    /// Helper function for exhaustive LHS
    /// Finds the lower bounds for each dimension based on the flat index of the hypercube in bounded
    /// space
    ///
    /// # Arguments
    /// * `i` - The index of the hypercube
    /// * `steps` - The step size for each dimension
    ///
    /// # Returns
    /// A vector of lower bounds for each dimension
    ///
    fn get_lower_bounds_by_index(&self, i: usize, steps: &[f64]) -> Vec<f64> {
        let n = self.input_domain.len();
        (0..n)
            .map(|d| {
                self.input_domain[d][0] as f64
                    + ((i / self.n_partitions.pow(d as u32)) % self.n_partitions) as f64 * steps[d]
            })
            .collect::<Vec<_>>()
    }

    fn compute_steps(&self) -> Vec<f64> {
        let n = self.input_domain.len();
        (0..n)
            .map(|d| {
                (self.input_domain[d][1] - self.input_domain[d][0]) as f64
                    / self.n_partitions as f64
            })
            .collect::<Vec<_>>()
    }

    fn initialise_randomised_point_indices(&mut self) {
        let n = self.input_domain.len();
        let n_points = self.n_partitions.pow(n as u32);
        assert!(
            n_points <= u32::MAX as usize,
            "Too many points to randomise with {} dimensions and {} partitions",
            n,
            self.n_partitions
        );
        self.randomised_point_indices = (0..n_points).map(|x| x as u32).collect::<Vec<u32>>();
        self.randomised_point_indices.shuffle(&mut rand::rng());
    }

    /// Populates a test case suite with random points using latin hypercube sampling
    ///
    /// From https://www.statisticshowto.com/latin-hypercube-sampling/:
    /// One-dimensional Latin hypercube sampling involves dividing your cumulative density function (cdf) into n equal partitions; and then choosing a random data point in each partition.
    ///
    /// As a simple example, let’s say you needed a random sample with 100 data points. First, divide the cdf into 100 equal intervals. If your distribution starts at 0 and ends with k , your first data point would be selected from the interval between (0,k/100). The second data point would be from the interval (k/100, 2k/100), your third from (2k/100, 3k/100), and so on. In each interval you would randomly select one point, giving you 100 different points.
    ///
    /// This is an exhaustive variant of LHS for multi-dimensional input domains, meaning that it generates a random point for each hypercube in random order.
    /// Thus, the total number of test cases required to cover each hypercube is n_partitions^n, where n is the number of dimensions.
    fn populate_test_cases_exhaustive_optimised(&mut self, existing_test_cases: &mut Vec<Point>) {
        let n = self.input_domain.len();
        let n_to_generate = 1000; // TODO: Move out of function
        let mut points = Vec::with_capacity(n_to_generate);

        let steps = self.compute_steps();

        (0..n_to_generate).for_each(|_| {
            if self.randomised_point_indices.is_empty() {
                self.initialise_randomised_point_indices();
            }
            let rand_index = self.randomised_point_indices.pop().unwrap() as usize;

            let lower_bounds = self.get_lower_bounds_by_index(rand_index, &steps);
            // println!("lower bounds: {:?}", lower_bounds);
            let point = Point {
                coordinates: (0..n)
                    .map(|d| {
                        let upper_bound = f32::min(
                            lower_bounds[d] as f32 + steps[d] as f32,
                            self.input_domain[d][1] as f32,
                        );
                        rand::random_range(lower_bounds[d] as f32..upper_bound)
                    })
                    .collect(),
                n,
            };
            points.push(point);
        });

        existing_test_cases.append(&mut points);
    }

    pub fn populate_test_cases(&mut self, existing_test_cases: &mut Vec<Point>) {
        match self.exhaustive {
            true => self.populate_test_cases_exhaustive_optimised(existing_test_cases),
            false => self.populate_test_cases_random(existing_test_cases),
        }
    }

    pub fn test_lhs_art_effectiveness(&'this mut self, fzb: &FaultZone) -> usize {
        let max_tries = (30.0 / fzb.get_theta()) as usize;
        let mut test_case_suite = Vec::with_capacity(self.n_partitions);

        self.populate_test_cases(&mut test_case_suite);

        let mut test_case_i = 0;

        while test_case_i < max_tries {
            // println!("lhs_art test_case_i: {}", test_case_i + 1);
            // check if the fault zone is found with current test case
            let test_case = &test_case_suite[test_case_i];
            if fzb.find_target(test_case) {
                break;
            }

            test_case_i += 1;

            // if all existing test cases are checked, generate new ones
            if test_case_i == test_case_suite.len() {
                self.populate_test_cases(&mut test_case_suite);
            }
        }

        if test_case_i == max_tries {
            println!("lhs art max tries ({max_tries}) reached")
        }

        // println!("done");
        test_case_i + 1
    }

    pub fn test_lhs_art_efficiency(
        &'this mut self,
        n_generated_values: usize,
        bound: &'this [Vec<i32>],
    ) {
        self.input_domain = bound; // Set input_domain
        let mut test_case_suite = Vec::with_capacity(n_generated_values);

        if n_generated_values <= self.n_partitions {
            self.populate_test_cases(&mut test_case_suite);
        } else {
            while test_case_suite.len() < n_generated_values {
                self.populate_test_cases(&mut test_case_suite);
            }
        }
    }
}

mod test {
    #[test]
    fn test_get_lower_bounds_by_index_2d() {
        use super::*;

        let input_domain = vec![vec![-10, 10], vec![-10, 10]];

        let mut lhs = LhsArt::new();
        lhs.input_domain = &input_domain;
        lhs.n_partitions = 4;

        let steps = lhs.compute_steps();

        let indices = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![
            vec![-10.0, -10.0],
            vec![-5.0, -10.0],
            vec![0.0, -10.0],
            vec![5.0, -10.0],
            vec![-10.0, -5.0],
            vec![-5.0, -5.0],
            vec![0.0, -5.0],
            vec![5.0, -5.0],
            vec![-10.0, 0.0],
        ];

        let actual = indices
            .iter()
            .map(|i| lhs.get_lower_bounds_by_index(*i, &steps))
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }
}
