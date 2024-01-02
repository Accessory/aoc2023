use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::grid_point::GridPoint;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Self { data: value }
    }
}

impl<T> Grid<T> {
    pub fn with_width_height(width: usize, height: usize, fill: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![vec![fill; width]; height],
        }
    }

    pub fn get_max_x(&self) -> usize {
        self.data.iter().map(|i| i.len()).max().unwrap_or(0)
    }

    pub fn get_max_y(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|i| i.get(x))
    }

    pub fn count_for(&self, value: &T) -> usize
    where
        T: PartialEq,
    {
        self.data.iter().flatten().filter(|i| *i == value).count()
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y][x] = value;
    }

    pub fn get_from_point(&self, point: &GridPoint) -> Option<&T> {
        self.get(point.x, point.y)
    }

    pub fn set_from_point(&mut self, point: &GridPoint, value: T) {
        self.set(point.x, point.y, value)
    }

    pub fn create_sub_grid(&self, x: usize, y: usize, width: usize, height: usize) -> Self
    where
        T: Copy,
    {
        let mut new_data: Vec<Vec<T>> = Vec::with_capacity(height);
        for y in y..y + width {
            let mut row = Vec::with_capacity(width);
            for x in x..x + width {
                let point = self.get(x, y).unwrap();
                row.push(*point);
            }
            new_data.push(row);
        }
        Grid { data: new_data }
    }

    pub fn get_sub_grid(&self, x: usize, y: usize, width: usize, height: usize) -> Grid<&T> {
        let mut new_data: Vec<Vec<&T>> = Vec::with_capacity(height);
        for y in y..y + width {
            let mut row = Vec::with_capacity(width);
            for x in x..x + width {
                let point = self.get(x, y).unwrap();
                row.push(point);
            }
            new_data.push(row);
        }
        Grid { data: new_data }
    }

    pub fn print_data(&self)
    where
        T: Display,
    {
        for row in self.data.iter() {
            for i in row.iter() {
                print!("{}", *i);
            }
            println!();
        }
    }

    pub fn grid_to_string(&self) -> String
    where
        T: Display,
    {
        let mut rtn = String::with_capacity(self.get_max_x() * self.get_max_y() + self.get_max_y());
        for row in self.data.iter() {
            for i in row.iter() {
                rtn.push_str(&format!("{}", *i));
            }
            rtn.push('\n');
        }
        rtn
    }

    pub fn get_all_positions_for(&self, value: &T) -> Vec<GridPoint>
    where
        T: std::cmp::PartialEq,
    {
        let mut rtn = Vec::new();
        for (y, row) in self.data.iter().enumerate() {
            for (x, column) in row.iter().enumerate() {
                if column == value {
                    rtn.push(GridPoint::new(x, y));
                }
            }
        }
        rtn
    }

    pub fn swap(&mut self, x_1: usize, y_1: usize, x_2: usize, y_2: usize)
    where
        T: Copy,
    {
        if y_1 == y_2 {
            self.data[y_1].swap(x_1, x_2);
        } else {
            let tmp = self.data[y_1][x_1];
            self.data[y_1][x_1] = self.data[y_2][x_2];
            self.data[y_2][x_2] = tmp;
        }
    }

    pub fn get_hash(&self) -> u64
    where
        T: Hash,
    {
        let mut default_hasher = DefaultHasher::new();
        self.hash(&mut default_hasher);
        default_hasher.finish()
    }

    pub fn find<F>(&self, find_func: F) -> Option<GridPoint>
    where
        F: Fn(&T) -> bool,
    {
        let max_x = self.get_max_x();
        let max_y = self.get_max_y();
        for y in 0..max_y {
            for x in 0..max_x {
                if find_func(&self.data[y][x]) {
                    return Some(GridPoint::new(x, y));
                }
            }
        }
        None
    }
}
