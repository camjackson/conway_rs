pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub neighbours: Vec<usize>,
    pub alive: bool,
}

impl Cell {
    pub fn update(&mut self, alive_neighbours: usize) {
        self.alive = match (self.alive, alive_neighbours) {
            (false, 3) => true,
            (true, 2) => true,
            (true, 3) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    fn new_cell(alive: bool) -> Cell {
        Cell { x: 0., y: 0., scale: 1., neighbours: Vec::new(), alive: alive }
    }

    #[test]
    fn it_dies_if_alive_with_less_than_2_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(1);
        assert!(cell.alive == false);
    }

    #[test]
    fn it_dies_if_alive_with_more_than_3_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(4);
        assert!(cell.alive == false);
    }

    #[test]
    fn it_lives_if_alive_with_2_or_3_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(2);
        assert!(cell.alive == true);
        cell.update(3);
        assert!(cell.alive == true);
    }

    #[test]
    fn it_is_born_if_dead_with_exactly_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(3);
        assert!(cell.alive == true);
    }

    #[test]
    fn it_stays_dead_if_dead_with_less_than_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(2);
        assert!(cell.alive == false);
    }

    #[test]
    fn it_stays_dead_if_dead_with_more_than_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(4);
        assert!(cell.alive == false);
    }
}
