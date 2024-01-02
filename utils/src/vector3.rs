#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {} z: {}", self.x, self.y, self.z)
    }
}

impl Vector3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    // fn manhatten_distance(&self, other: &Vector3) -> i64 {
    //     manhatten_distance_3d(self.x, other.x, self.y, other.y, self.z, other.z)
    // }

    // fn manhatten_distance_to(&self, x: i64, y: i64, z: i64) -> i64 {
    //     manhatten_distance_3d(self.x, x, self.y, y, self.z, z)
    // }
    pub fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
