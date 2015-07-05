use cell::Cell;
use seeds::Seed;

pub struct Grid {
    pub cells: Vec<Cell>,
    pub alive_neighbours: Vec<usize>,
}

impl Grid {
    pub fn new(seed: Seed, width: i16, height: i16, square_size: f32) -> Grid {
        let mut cells = Vec::with_capacity((width * height) as usize);

        for y in (0..height) {
            for x in (0..width) {
                cells.push(Cell {
                    x: (x as f32 * square_size + square_size / 2.),
                    y: -(y as f32 * square_size + square_size / 2.),
                    scale: square_size,
                    neighbours: [
                        (x-1, y-1), (x, y-1), (x+1, y-1),
                        (x-1, y  ),           (x+1, y  ),
                        (x-1, y+1), (x, y+1), (x+1, y+1)
                    ].iter().map(|n| coords_to_index(*n, width, height)).collect(),
                    alive: seed(x, y),
                });
            }
        }
        Grid { 
            cells: cells,
            alive_neighbours: Vec::new() 
        }
    }

    pub fn update(&mut self) {
        self.alive_neighbours.clear();
        let cells = &mut self.cells;
        
        for cell in cells.iter() {
            self.alive_neighbours.push(cell.neighbours.iter().filter(|n| unsafe { cells.get_unchecked(**n) }.alive).count())
        }

        for (cell, cell_alive_neighbours) in cells.iter_mut().zip(self.alive_neighbours.iter()) {
            cell.update(*cell_alive_neighbours)
        }
    }
}

fn coords_to_index(coords: (i16, i16), grid_width: i16, grid_height: i16) -> usize {
    let (x, y) = coords;
    let x_wrapped = ((x + grid_width) % grid_width) as usize;
    let y_wrapped = ((y + grid_height) % grid_height) as usize;
    x_wrapped + (y_wrapped * grid_width as usize)
}

#[cfg(test)]
mod tests {
    use super::coords_to_index;

    #[test]
    fn it_returns_the_x_value_on_the_first_row() {
        assert!(coords_to_index((3, 0), 5, 3) == 3)
    }

    #[test]
    fn it_wraps_overflowing_x_values() {
        assert!(coords_to_index((6, 0), 5, 3) == 1)
    }

    #[test]
    fn it_wraps_underflowing_x_values() {
        assert!(coords_to_index((-1, 0), 5, 3) == 4)
    }

    #[test]
    fn it_adds_one_width_for_each_row() {
        assert!(coords_to_index((2, 2), 5, 3) == 12)
    }

    #[test]
    fn it_wraps_overflowing_y_values() {
        assert!(coords_to_index((1, 5), 5, 3) == 11)
    }

    #[test]
    fn it_wraps_underflowing_y_values() {
        assert!(coords_to_index((4, -2), 5, 3) == 9)
    }
}
