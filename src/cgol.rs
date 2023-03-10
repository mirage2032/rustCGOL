use rand::Rng;

pub struct CGOL {
    width: usize,
    height: usize,
    cell_matrix: Vec<bool>,
    neighbour_matrix: Vec<u8>,
}

impl CGOL {
    pub fn new(width: usize, height: usize) -> Self {
        CGOL {
            width,
            height,
            cell_matrix: vec![false; width * height],
            neighbour_matrix: vec![0; width * height],
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.size() {
            self.cell_matrix[i] = rng.gen();
        }
    }

    pub fn step(&mut self) {
        self.count_neighbours();    //Create neighbourhood map
        self.nghmap_to_cellmap();   //Advance cellmap based on the neighbourhood one
    }

    pub fn get_cmat_as_ascii(&self) -> String {
        let mut ascii_cell_matrix = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let current_idx = self.coord_to_idx(x, y);
                match self.cell_matrix[current_idx] {
                    true => ascii_cell_matrix.push('X'),
                    false => ascii_cell_matrix.push(' ')
                }
            }
            ascii_cell_matrix += &y.to_string();
            if y != self.height - 1 { ascii_cell_matrix.push('\n') };
        }
        ascii_cell_matrix
    }
    fn size(&self) -> usize {
        self.width * self.height
    }

    fn coord_to_idx(&self, x: usize, y: usize) -> usize {
        x + self.width * y
    }

    fn count_neighbours(&mut self) {
        self.neighbour_matrix = vec![0; self.size()];
        for main_y in 0..self.height {
            for main_x in 0..self.width {
                let main_idx = self.coord_to_idx(main_x, main_y);
                if !self.cell_matrix[main_idx] { continue; } //if cell is inactive don't increase neighbor count
                for neighbour_x in main_x.saturating_sub(1)..(main_x + 2).min(self.width) {
                    for neighbour_y in main_y.saturating_sub(1)..(main_y + 2).min(self.height) {
                        if main_x == neighbour_x && main_y == neighbour_y { continue; }
                        let neighbour_idx = self.coord_to_idx(neighbour_x, neighbour_y);
                        self.neighbour_matrix[neighbour_idx] += 1;
                    }
                }
            }
        }
    }

    fn nghmap_to_cellmap(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let current_idx = self.coord_to_idx(x, y);
                let cell = &mut self.cell_matrix[current_idx];
                let neighbours = self.neighbour_matrix[current_idx];
                if *cell && (neighbours < 2 || neighbours > 3) {
                    *cell = false;
                } else if !*cell && neighbours == 3 {
                    *cell = true;
                }
            }
        }
    }
}