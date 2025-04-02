use super::point::Point;

#[derive(Debug, Clone)]
pub struct Node {
    pub split: usize,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
    pub parent: Option<Box<Self>>,
    pub point: Option<Point>,
    pub boundary: Option<Vec<Vec<f64>>>,
    pub deep: i32,
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            split: 0,
            left: None,
            right: None,
            parent: None,
            point: None,
            boundary: None,
            deep: 0,
        }
    }
}
