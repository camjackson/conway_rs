use seeds;

pub type Location = (i16, i16);

#[derive(PartialEq, Eq, Clone)]
pub struct Grid {
    width: u16,
    height: u16,
    cells: Vec<bool>,
}

impl Grid {
    pub fn new(seed: seeds::Seed, width: u16, height: u16) -> Grid {
        let mut cells = Vec::with_capacity((width * height) as usize);

        for y in (0 .. height) {
            for x in (0 .. width) {
                cells.push(seed(x as i16, y as i16));
            }
        }
        Grid {
            width: width,
            height: height,
            cells: cells
        }
    }

    pub fn update(&mut self) {
        let mut alive_neighbours = vec![0; self.width as usize * self.height as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                let location = (x as i16, y as i16);
                alive_neighbours[self.idx(location)] = self.alive_neighbours(location);
            }
        }

        for (cell, alive_neighbours) in self.cells.iter_mut().zip(alive_neighbours.into_iter()) {
            *cell = match (*cell, alive_neighbours) {
                (false, 3) => true,
                (true, 2) => true,
                (true, 3) => true,
                _ => false,
            };
        }
    }

    #[inline]
    pub fn width(&self) -> u16 { self.width }
    #[inline]
    pub fn height(&self) -> u16 { self.height }

    pub fn get(&self, location: Location) -> bool {
        let idx = self.idx(location);
        self.cells[idx]
    }

    fn neighbours_of(&self, location: Location) -> [Location; 8] {
        let (x, y) = location;
        [
            (x-1, y-1), (x, y-1), (x+1, y-1),
            (x-1, y  ),           (x+1, y  ),
            (x-1, y+1), (x, y+1), (x+1, y+1)
        ]
    }

    fn alive_neighbours(&self, location: Location) -> usize {
        self.neighbours_of(location).iter().filter(|&&loc| self.get(loc)).count()
    }

    fn idx(&self, location: Location) -> usize {
        let (x, y) = location;
        let width = self.width as i16;
        let height = self.height as i16;
        let x = (x + width) % width;
        let y = (y + height) % height;
        (x + (y * width)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_grid() -> Grid {
        fn my_seed(x: i16, y: i16) -> bool {
            match (x, y) {
                (0, 0) => true,
                _ => false,
            }
        }

        Grid::new(my_seed, 5, 3)
    }

    #[test]
    fn it_returns_the_x_value_on_the_first_row() {
        assert_eq!(3, default_grid().idx((3, 0)))
    }

    #[test]
    fn it_wraps_overflowing_x_values() {
        assert_eq!(1, default_grid().idx((6,0)))
    }

    #[test]
    fn it_wraps_underflowing_x_values() {
        assert_eq!(4, default_grid().idx((-1, 0)))
    }

    #[test]
    fn it_adds_one_width_for_each_row() {
        assert_eq!(12, default_grid().idx((2, 2)))
    }

    #[test]
    fn it_wraps_overflowing_y_values() {
        assert_eq!(11, default_grid().idx((1, 5)))
    }

    #[test]
    fn it_wraps_underflowing_y_values() {
        assert_eq!(9, default_grid().idx((4, -2)))
    }
}
