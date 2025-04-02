use crate::{fault::fault_zone::FaultZone, util::point::Point};

#[derive(Debug)]
pub struct Rt<'this> {
    pub input_domain: &'this [Vec<i32>],
}

impl Default for Rt<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'this> Rt<'this> {
    pub fn new() -> Rt<'this> {
        Rt { input_domain: &[] }
    }

    pub fn test_rt_effectiveness(
        &'this mut self,
        bound: &'this [Vec<i32>],
        fzb: &FaultZone,
    ) -> usize {
        self.input_domain = bound; // Set input bounds

        let max_tries = (30.0 / fzb.get_theta()) as usize;
        let mut n_generated = 0;

        while n_generated < max_tries {
            // check if the fault zone is found with current test case
            let test_case = Point::generate_rand_p(bound);
            n_generated += 1;
            if fzb.find_target(&test_case) {
                break;
            }

            // println!("random art test_case_i: {}", test_case_i);
        }

        if n_generated == max_tries {
            println!("random art max tries ({max_tries}) reached");
        }
        // println!("done");

        n_generated
    }

    pub fn test_random_art_efficiency(
        &'this mut self,
        n_generated_values: i32,
        bound: &'this [Vec<i32>],
    ) {
        self.input_domain = bound; // Set input_domain

        for _i in 0..n_generated_values {
            Point::generate_rand_p(bound);
        }
    }
}
