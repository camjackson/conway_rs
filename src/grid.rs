pub struct Cell {
    pub x: f32,
    pub y: f32,
}

pub fn new(width: i16, height: i16, square_size: f32) -> Vec<Cell> {
    let mut data = Vec::new();
    for y in (0i16 .. height) {
        for x in (0i16 .. width) {
            data.push(Cell {
                x: (x - width / 2) as f32 * square_size,
                y: (y - height / 2) as f32 * square_size
            });
        }
    }
    data
}
