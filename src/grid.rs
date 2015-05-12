use rand;

use cell::Cell;

pub struct Grid {
    pub cells: Vec<Cell>,
    width: i16,
    height: i16,
}

pub fn new(width: i16, height: i16, square_size: f32) -> Grid {
    let mut cells = Vec::new();
    for y in (0i16 .. height) {
        for x in (0i16 .. width) {
            cells.push(Cell {
                x: (x - width / 2) as f32 * square_size,
                y: (y - height / 2) as f32 * square_size,
                neighbours: vec![
                    (x-1, y-1), (x, y-1), (x+1, y-1),
                    (x-1, y  ),           (x+1, y  ),
                    (x-1, y+1), (x, y+1), (x+1, y+1)
                ],
                alive: starts_alive(x, y),
            });
        }
    }
    Grid{ cells: cells, width: width, height: height }
}

fn starts_alive(x: i16, y: i16) -> bool {
    //change here to switch between seeds
    gosper_glider(x, y)
}

fn random(_: i16, _: i16) -> bool {
    rand::random::<u8>() % 2 == 0
}

fn diehard(x: i16, y: i16) -> bool {
    match (x, y) {
        (31, 22) => true,
        (32, 21) => true,
        (32, 22) => true,
        (36, 21) => true,
        (37, 21) => true,
        (38, 21) => true,
        (37, 23) => true,
        _ => false
    }
}

fn gosper_glider(x: i16, y: i16) -> bool {
    match (x, y) {
        (1, 4) => true,
        (1, 5) => true,
        (2, 4) => true,
        (2, 5) => true,

        (11, 3) => true,
        (11, 4) => true,
        (11, 5) => true,

        (12, 2) => true,
        (12, 6) => true,

        (13, 1) => true,
        (13, 7) => true,
        (14, 1) => true,
        (14, 7) => true,

        (15, 4) => true,

        (16, 2) => true,
        (16, 6) => true,

        (17, 3) => true,
        (17, 4) => true,
        (17, 5) => true,

        (18, 4) => true,

        (21, 5) => true,
        (21, 6) => true,
        (21, 7) => true,

        (22, 5) => true,
        (22, 6) => true,
        (22, 7) => true,

        (23, 4) => true,
        (23, 8) => true,

        (25, 3) => true,
        (25, 4) => true,
        (25, 8) => true,
        (25, 9) => true,

        (35, 6) => true,
        (35, 7) => true,
        (36, 6) => true,
        (36, 7) => true,

        _ => false
    }
}

impl Grid {
    pub fn update(&mut self) {
        let mut alive_neighbours = Vec::new();

        for cell in self.cells.iter() {
            let mut alive_count = 0;
            for neighbour in cell.neighbours.iter() {
                if self.cells[self.coords_to_index(*neighbour)].alive {
                    alive_count += 1;
                }
            }
            alive_neighbours.push(alive_count)
        }
//        for cell in self.cells.iter() {
//            alive_neighbours.push(cell.neighbours.iter().filter(|n| self.cells[self.coords_to_index(**n)].alive).count())
//        }

        for (cell, cell_alive_neighbours) in self.cells.iter_mut().zip(alive_neighbours.iter()) {
            cell.update(*cell_alive_neighbours)
        }
    }

    fn coords_to_index(&self, coords: (i16, i16)) -> usize {
        let (x, y) = coords;
        // assumes that x and y won't underflow more than once
        let x_wrapped = (x + self.width) % self.width;
        let y_wrapped = (y + self.height) % self.height;
        (x_wrapped + (y_wrapped * self.width)) as usize
    }
}


#[cfg(test)]
mod tests {
    use super::coords_to_index;

    fn new_grid() {
        Grid{ cells: Vec::new(), width: 5, height: 3 }
    }

    #[test]
    fn it_returns_the_x_value_on_the_first_row() {
        assert!(new_grid().coords_to_index((3, 0)) == 3)
    }

    #[test]
    fn it_wraps_overflowing_x_values() {
        assert!(new_grid().coords_to_index((6, 0)) == 1)
    }

    #[test]
    fn it_wraps_underflowing_x_values() {
        assert!(new_grid().coords_to_index((-1, 0)) == 4)
    }

    #[test]
    fn it_adds_one_width_for_each_row() {
        assert!(new_grid().coords_to_index((2, 2)) == 12)
    }

    #[test]
    fn it_wraps_overflowing_y_values() {
        assert!(new_grid().coords_to_index((1, 5)) == 11)
    }

    #[test]
    fn it_wraps_underflowing_y_values() {
        assert!(new_grid().coords_to_index((4, -2)) == 9)
    }
}
