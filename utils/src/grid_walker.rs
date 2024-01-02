use crate::{grid::Grid, grid_direction::GridDirection, grid_point::GridPoint};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GridWalker {
    pub position: GridPoint,
    pub direction: GridDirection,
}

impl GridWalker {
    pub fn new(position: GridPoint, direction: GridDirection) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn r#move(&mut self) {
        self.position.move_direction(&self.direction)
    }
    pub fn move_with_check(&mut self, max_x: usize, max_y: usize) -> bool {
        let rtn = match self.direction {
            GridDirection::Up => self.position.y != 0,
            GridDirection::Right => self.position.x + 1 != max_y,
            GridDirection::Down => self.position.y + 1 != max_x,
            GridDirection::Left => self.position.x != 0,
        };
        if rtn {
            self.position.move_direction(&self.direction)
        }
        rtn
    }

    pub fn get_from_grid(&self, grid: &Grid<char>) -> Option<char> {
        grid.get_from_point(&self.position).copied()
    }
}
