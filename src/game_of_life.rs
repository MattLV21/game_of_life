use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct ConwaysGame {
    cells: Vec<Cell>,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl ConwaysGame {
    pub fn new(starting: Vec<Cell>) -> Self {
        Self {
            cells: starting,
        }
    }

    pub fn get_living_cells(&self) -> Vec<Cell> {
        return self.cells.clone();
    }

    pub fn calculate_next_gen_map(&self) -> HashMap<Cell, u8> {
        let mut potential_cells: HashMap<Cell, u8> = HashMap::new();
        for cell in self.cells.iter() {
            for x in -1..2 {
                for y in -1..2 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    let key = Cell::new(cell.x + x, cell.y + y);
                    *potential_cells.entry(key).or_insert(0) += 1;
                }
            }
        }
        return potential_cells;
    }

    pub fn apply_gen_from_map(&mut self, map: &HashMap<Cell, u8>) {
        let mut new_gen: Vec<Cell> = Vec::new();
        
        for (cell, &count) in map.iter() {
            if self.cells.contains(cell) {
                // Cell is alive
                if count == 2 || count == 3 {
                    new_gen.push(cell.clone()); // survives
                }
            } else {
                // Cell is dead
                if count == 3 {
                    new_gen.push(cell.clone()); // becomes alive
                }
            }
        }
    
        self.cells = new_gen;
    }
}