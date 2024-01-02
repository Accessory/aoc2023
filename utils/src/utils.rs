use std::ops::Add;

pub fn manhatten_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    x1.abs_diff(x2).add(y1.abs_diff(y2)) as i64
}

pub fn manhatten_distance_3d(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    x1.abs_diff(x2).add(y1.abs_diff(y2)).add(z1.abs_diff(z2)) as i64
}

pub fn manhatten_distance_3d_from_zero(x: i64, y: i64, z: i64) -> i64 {
    x.abs().add(y.abs()).add(z.abs())
}
