use crate::{
    fault::fault_zone::FaultZone,
    util::{node::Node, point::Point},
};

#[derive(Debug)]
pub struct KdfcArt<'this> {
    pub root: Node,
    pub size: usize,
    pub candidate_num: i32,
    pub input_domain: &'this [Vec<i32>],
}

impl Default for KdfcArt<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'this> KdfcArt<'this> {
    pub fn new() -> Self {
        Self {
            root: Node::new(),
            size: 0,
            candidate_num: 10,
            input_domain: &[],
        }
    }

    pub fn with_bound(bound: &'this [Vec<i32>]) -> Self {
        let mut kda = Self::new();
        kda.input_domain = bound;
        kda.root.boundary = Some(vec![vec![0.0; 2]; bound.len()]);

        (0..bound.len()).for_each(|i| {
            kda.root.boundary.as_mut().unwrap()[i][0] = bound[i][0] as f64;
            kda.root.boundary.as_mut().unwrap()[i][1] = bound[i][1] as f64;
        });

        kda
    }

    pub fn get_tree_path(&self, point: &Point) -> Vec<&Node> {
        let mut path = Vec::new();
        let mut path_node = &self.root;

        loop {
            path.push(path_node);
            if let Some(path_node_point) = path_node.point.as_ref() {
                let split = path_node.split;
                if path_node_point.coordinates[split] > point.coordinates[split] {
                    if path_node.left.is_none() {
                        break;
                    }
                    path_node = path_node.left.as_ref().unwrap();
                } else {
                    if path_node.right.is_none() {
                        break;
                    }
                    path_node = path_node.right.as_ref().unwrap();
                }
            } else {
                break;
            }
        }
        path
    }

    pub fn judge_direction(&self, p: &Point, node: &Node) -> i32 {
        if p.coordinates[node.split] < node.point.as_ref().unwrap().coordinates[node.split] {
            0
        } else {
            1
        }
    }

    pub fn get_min_dis_by_all(&self, p: &Point) -> f64 {
        let path = self.get_tree_path(p);
        let mut distance = f64::MAX;

        for path_node in path.iter().rev() {
            if self.is_cross_split_line(p, distance, path_node) {
                let d = Point::get_distance(p, path_node.point.as_ref().unwrap());
                if distance > d as f64 {
                    distance = d as f64;
                }

                let direction = self.judge_direction(p, path_node);

                let temp_node = if direction == 0 {
                    path_node.right.as_ref()
                } else {
                    path_node.left.as_ref()
                };

                if let Some(temp_node) = temp_node {
                    let mut queue = Vec::new();
                    queue.push(temp_node);

                    while let Some(temp_node) = queue.pop() {
                        let direction = self.judge_direction(p, temp_node);
                        if self.is_cross_split_line(p, distance, temp_node) {
                            let d = Point::get_distance(p, temp_node.point.as_ref().unwrap());
                            if distance > d as f64 {
                                distance = d as f64;
                            }

                            if direction == 1 {
                                if let Some(left) = &temp_node.left {
                                    queue.push(left);
                                }
                            } else if let Some(right) = &temp_node.right {
                                queue.push(right);
                            }
                        }

                        if direction == 0 {
                            if let Some(left) = &temp_node.left {
                                queue.push(left);
                            }
                        } else if let Some(right) = &temp_node.right {
                            queue.push(right);
                        }
                    }
                }
            }
        }
        distance
    }

    pub fn get_min_dis_by_backtracking(&self, p: &Point, back: i32) -> f64 {
        let mut num = 0;
        let path = self.get_tree_path(p);
        let mut distance = f64::MAX;

        for path_node in path.iter().rev() {
            if self.is_cross_split_line(p, distance, path_node) {
                let d = Point::get_distance(p, path_node.point.as_ref().unwrap());
                if distance > d as f64 {
                    distance = d as f64;
                }
                num += 1;
                if num == back {
                    return distance;
                }

                let direction = self.judge_direction(p, path_node);
                let temp_node = if direction == 0 {
                    path_node.right.as_ref()
                } else {
                    path_node.left.as_ref()
                };

                if let Some(temp_node) = temp_node {
                    let mut queue = Vec::new();
                    queue.push(temp_node);

                    while let Some(temp_node) = queue.pop() {
                        let direction = self.judge_direction(p, temp_node);
                        if self.is_cross_split_line(p, distance, temp_node) {
                            let d = Point::get_distance(p, temp_node.point.as_ref().unwrap());
                            if distance > d as f64 {
                                distance = d as f64;
                            }
                            num += 1;
                            if num == back {
                                return distance;
                            }

                            if direction == 1 {
                                if let Some(left) = &temp_node.left {
                                    queue.push(left);
                                }
                            } else if let Some(right) = &temp_node.right {
                                queue.push(right);
                            }
                        }

                        if direction == 0 {
                            if let Some(left) = &temp_node.left {
                                queue.push(left);
                            }
                        } else if let Some(right) = &temp_node.right {
                            queue.push(right);
                        }
                    }
                }
            } else {
                num += 1;
                if num == back {
                    return distance;
                }
            }
        }
        distance
    }

    pub fn is_cross_split_line(&self, p: &Point, distance: f64, node: &Node) -> bool {
        let node_p = node.point.as_ref().unwrap();
        if (node_p.coordinates[node.split] - p.coordinates[node.split]).abs() as f64 >= distance {
            return false;
        }
        true
    }

    pub fn insert_point_by_strategy(&mut self, p: &Point) {
        if self.root.point.is_none() {
            self.root.deep = 1;
            self.root.point = Some(p.clone());
            self.root.boundary = Some(vec![vec![0.0; 2]; p.n]);
            for i in 0..p.n {
                self.root.boundary.as_mut().unwrap()[i][0] = self.input_domain[i][0] as f64;
                self.root.boundary.as_mut().unwrap()[i][1] = self.input_domain[i][1] as f64;
            }
            self.root.split = Self::split_select(self.root.boundary.as_ref().unwrap(), p);
        } else {
            let mut ntemp = &mut self.root;
            let mut n = &mut Node::new();

            while let Some(ntemp_p) = ntemp.point.as_ref() {
                if ntemp_p.coordinates[ntemp.split] > p.coordinates[ntemp.split] {
                    if ntemp.left.is_none() {
                        ntemp.left = Some(Box::new(Node::new()));
                        n = ntemp.left.as_mut().unwrap();
                        break;
                    }
                    ntemp = ntemp.left.as_mut().unwrap();
                } else {
                    if ntemp.right.is_none() {
                        ntemp.right = Some(Box::new(Node::new()));
                        n = ntemp.right.as_mut().unwrap();
                        break;
                    }
                    ntemp = ntemp.right.as_mut().unwrap();
                }
            }

            n.point = Some(p.clone());
            n.boundary = Some(vec![vec![0.0; 2]; p.n]);
            n.deep = ntemp.deep + 1;
            for i in 0..p.n {
                n.boundary.as_mut().unwrap()[i][0] = ntemp.boundary.as_ref().unwrap()[i][0];
                n.boundary.as_mut().unwrap()[i][1] = ntemp.boundary.as_ref().unwrap()[i][1];
            }
            if n.point.as_ref().unwrap().coordinates[ntemp.split]
                < ntemp.point.as_ref().unwrap().coordinates[ntemp.split]
            {
                n.boundary.as_mut().unwrap()[ntemp.split][1] =
                    ntemp.point.as_ref().unwrap().coordinates[ntemp.split] as f64;
            } else {
                n.boundary.as_mut().unwrap()[ntemp.split][0] =
                    ntemp.point.as_ref().unwrap().coordinates[ntemp.split] as f64;
            }
            let boundary = n.boundary.as_ref().unwrap();
            n.split = Self::split_select(boundary, p);
        }
        self.size += 1;
    }

    pub fn insert_point_by_turn(&mut self, p: &Point) {
        if self.root.point.is_none() {
            self.root.point = Some(p.clone());
            self.root.split = 0;
            self.root.deep = 1;
        } else {
            let mut ntemp = &mut self.root;
            let n = loop {
                let ntemp_p = ntemp.point.as_ref().unwrap();

                if ntemp_p.coordinates[ntemp.split] > p.coordinates[ntemp.split] {
                    if ntemp.left.is_none() {
                        ntemp.left = Some(Box::new(Node::new()));
                        break ntemp.left.as_mut().unwrap();
                    }
                    ntemp = ntemp.left.as_mut().unwrap();
                } else {
                    if ntemp.right.is_none() {
                        ntemp.right = Some(Box::new(Node::new()));
                        break ntemp.right.as_mut().unwrap();
                    }
                    ntemp = ntemp.right.as_mut().unwrap();
                }
            };

            n.point = Some(p.clone());
            n.deep = ntemp.deep + 1;
            if ntemp.split == (p.coordinates.len() - 1) {
                n.split = 0;
            } else {
                n.split = ntemp.split + 1;
            }
        }
        self.size += 1;
    }

    pub fn test_naive_kdfc_effectiveness(&mut self, fault_zone: &FaultZone) {
        let p = Point::generate_rand_p(self.input_domain);
        self.insert_point_by_turn(&p);
        if fault_zone.find_target(&p) {
            return;
        }

        loop {
            let mut can_d = Vec::new();
            for _ in 0..self.candidate_num {
                can_d.push(Point::generate_rand_p(self.input_domain));
            }

            let mut final_case = &can_d[0];
            let mut distance = self.get_min_dis_by_all(final_case);

            (1..can_d.len()).for_each(|c| {
                let d = self.get_min_dis_by_all(&can_d[c]);
                if distance < d {
                    distance = d;
                    final_case = &can_d[c];
                }
            });

            self.insert_point_by_turn(final_case);
            // if (self.size % 1000) == 0 {
            //     println!("final_case: {:?}", final_case);
            //     println!("size: {}", self.size);
            // }
            if fault_zone.find_target(final_case) {
                break;
            }
        }
    }

    // pub fn test_semi_bal_kdfc_effectiveness(&mut self, fzb: &dyn FaultZone) {
    pub fn test_semi_bal_kdfc_effectiveness(&mut self, fzb: &FaultZone) {
        let p = Point::generate_rand_p(self.input_domain);
        self.insert_point_by_strategy(&p);
        if fzb.find_target(&p) {
            return;
        }

        loop {
            let mut can_d = Vec::new();
            for _ in 0..self.candidate_num {
                can_d.push(Point::generate_rand_p(self.input_domain));
            }

            let mut final_case = &can_d[0];
            let mut distance = self.get_min_dis_by_all(final_case);

            (1..can_d.len()).for_each(|c| {
                let d = self.get_min_dis_by_all(&can_d[c]);
                if distance < d {
                    distance = d;
                    final_case = &can_d[c];
                }
            });

            self.insert_point_by_strategy(final_case);
            // if (self.size % 1000) == 0 {
            //     println!("final_case: {:?}", final_case);
            //     println!("size: {}", self.size);
            // }
            if fzb.find_target(final_case) {
                break;
            }
        }
    }

    pub fn test_lim_bal_kdfc_effectiveness(&mut self, fzb: &FaultZone, back_num: &[i32]) {
        let p = Point::generate_rand_p(self.input_domain);
        self.insert_point_by_strategy(&p);
        if fzb.find_target(&p) {
            return;
        }

        loop {
            let mut can_d = Vec::new();
            for _ in 0..self.candidate_num {
                can_d.push(Point::generate_rand_p(self.input_domain));
            }

            let mut final_case = &can_d[0];
            let back = back_num[self.size];
            let mut distance = self.get_min_dis_by_backtracking(final_case, back);

            (1..can_d.len()).for_each(|c| {
                let d = self.get_min_dis_by_backtracking(&can_d[c], back);
                if distance < d {
                    distance = d;
                    final_case = &can_d[c];
                }
            });

            self.insert_point_by_strategy(final_case);
            // if (self.size % 1000) == 0 {
            //     println!("final_case: {:?}", final_case);
            //     println!("size: {}", self.size);
            // }
            if fzb.find_target(final_case) {
                break;
            }
        }
    }

    pub fn test_naive_kdfc_efficiency(&mut self, point_num: i32) {
        let p = Point::generate_rand_p(self.input_domain);
        self.insert_point_by_turn(&p);

        for _ in 1..point_num {
            let mut can_d = Vec::new();
            for _ in 0..self.candidate_num {
                can_d.push(Point::generate_rand_p(self.input_domain));
            }

            let mut final_case = can_d[0].clone();
            let mut distance = self.get_min_dis_by_all(&final_case);

            (1..can_d.len()).for_each(|c| {
                let d = self.get_min_dis_by_all(&can_d[c]);
                if distance < d {
                    distance = d;
                    final_case = can_d[c].clone();
                }
            });

            self.insert_point_by_turn(&final_case);
        }
    }

    pub fn test_semi_bal_kdfc_efficiency(&mut self, point_num: i32) {
        let p = Point::generate_rand_p(self.input_domain);
        self.insert_point_by_strategy(&p);

        for _ in 1..point_num {
            let mut can_d = Vec::new();
            for _ in 0..self.candidate_num {
                can_d.push(Point::generate_rand_p(self.input_domain));
            }

            let mut final_case = can_d[0].clone();
            let mut distance = self.get_min_dis_by_all(&final_case);

            (1..can_d.len()).for_each(|c| {
                let d = self.get_min_dis_by_all(&can_d[c]);
                if distance < d {
                    distance = d;
                    final_case = can_d[c].clone();
                }
            });

            self.insert_point_by_strategy(&final_case);
        }
    }

    pub fn test_lim_bal_kdfc_efficiency(&mut self, point_num: i32, back_num: &[i32]) {
        let p = Point::generate_rand_p(self.input_domain);
        self.insert_point_by_strategy(&p);

        for _ in 1..point_num {
            let mut can_d = Vec::new();
            for _ in 0..self.candidate_num {
                can_d.push(Point::generate_rand_p(self.input_domain));
            }

            let mut final_case = can_d[0].clone();
            let back = back_num[self.size];
            let mut distance = self.get_min_dis_by_backtracking(&final_case, back);

            (1..can_d.len()).for_each(|c| {
                let d = self.get_min_dis_by_backtracking(&can_d[c], back);
                if distance < d {
                    distance = d;
                    final_case = can_d[c].clone();
                }
            });

            self.insert_point_by_strategy(&final_case);
        }
    }

    pub fn split_select(boundary: &[Vec<f64>], p: &Point) -> usize {
        let mut rate = 0.0;
        let mut split = 0;

        (0..p.n).for_each(|i| {
            let length = boundary[i][1] - boundary[i][0];
            let lx1 = boundary[i][1] - p.coordinates[i] as f64;
            let lx2 = p.coordinates[i] as f64 - boundary[i][0];
            let spread =
                length * (1.0 - (lx1 / length) * (lx1 / length) - (lx2 / length) * (lx2 / length));
            if rate < spread {
                rate = spread;
                split = i;
            }
        });
        split
    }
}
