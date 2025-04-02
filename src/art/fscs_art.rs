use crate::{fault::fault_zone::FaultZone, util::point::Point};

#[derive(Debug)]
pub struct FscsArt<'this> {
    pub cand_num: i32,
    pub input_domain: &'this [Vec<i32>],
}

impl Default for FscsArt<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'this> FscsArt<'this> {
    pub fn new() -> FscsArt<'this> {
        FscsArt {
            cand_num: 10,
            input_domain: &[]
        }
    }

    pub fn with_n(n: i32) -> Self {
        Self {
            cand_num: n,
            input_domain: &[],
        }
    }

    pub fn find_furthest_candidate(&self, tcp: &[Point], size: usize, cand_p: &[Point]) -> usize {
        let mut dist = vec![0.0; cand_p.len()];
        let mut furthest_dist = 0.0;
        let mut furthest_index = 0;

        (0..cand_p.len()).for_each(|i| {
            dist[i] = Point::get_distance(&cand_p[i], &tcp[0]);
            (1..size).for_each(|j| {
                let temp_dist = Point::get_distance(&cand_p[i], &tcp[j]);
                if temp_dist < dist[i] {
                    dist[i] = temp_dist;
                }
            });
            if i == 0 {
                furthest_dist = dist[0];
                furthest_index = 0;
            } else if furthest_dist < dist[i] {
                furthest_dist = dist[i];
                furthest_index = i;
            }
        });
        furthest_index
    }

    pub fn test_fscs_art_effectiveness(&'this mut self, bound: &'this [Vec<i32>], fzb: &FaultZone) -> i32 {
        self.input_domain = bound; // Set input_domain
        let mut generated_num = 0;
        let max_try = (30.0 / fzb.get_theta()) as i32;
        // println!("max_try: {max_try}");
        let mut tcp = Vec::with_capacity((max_try + 2) as usize);
        let mut cand_p = Vec::with_capacity(self.cand_num as usize);

        tcp.push(Point::generate_rand_p(bound));
        // println!("tcp[0]: {:?}", tcp[0]);
        generated_num += 1;

        loop {
            cand_p.clear();
            for _ in 0..self.cand_num {
                cand_p.push(Point::generate_rand_p(bound));
            }

            let selected = self.find_furthest_candidate(&tcp, generated_num as usize, &cand_p);
            tcp.push(cand_p[selected].clone());
            generated_num += 1;


            // if (generated_num % 1000) == 0 {
            //     println!("generated_num: {generated_num}");
            // }

            if fzb.find_target(&tcp[generated_num as usize - 1]) {
                break;
            }

            if generated_num >= max_try {
                break;
            }
        }
        // println!("generated_num: {generated_num}");
        generated_num
    }

    pub fn test_fscs_art_efficiency(&'this mut self, num: i32, bound: &'this [Vec<i32>]) {
        self.input_domain = bound; // Set input_domain

        let mut tcp = Vec::with_capacity(num as usize);
        let mut cand_p = Vec::with_capacity(self.cand_num as usize);

        tcp.push(Point::generate_rand_p(bound));

        for j in 1..num {
            cand_p.clear();
            for _ in 0..self.cand_num {
                cand_p.push(Point::generate_rand_p(bound));
            }
            let selected = self.find_furthest_candidate(&tcp, j as usize, &cand_p);
            tcp.push(cand_p[selected].clone());
        }
    }
}
