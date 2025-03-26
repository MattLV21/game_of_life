use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LivingCell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct ConwaysGame {
    cells: Vec<LivingCell>,
}

impl LivingCell {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl ConwaysGame {
    pub fn new(starting: Vec<LivingCell>) -> Self {
        Self {
            cells: starting,
        }
    }

    pub fn get_living_cells(&self) -> Vec<LivingCell> {
        return self.cells.clone();
    }

    pub fn calculate_next_gen_map(&self) -> HashMap<LivingCell, u8> {
        let mut potential_cells: HashMap<LivingCell, u8> = HashMap::new();
        for cell in self.cells.iter() {
            for x in -1..2 {
                for y in -1..2 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    let key = LivingCell::new(cell.x + x, cell.y + y);
                    *potential_cells.entry(key).or_insert(0) += 1;
                }
            }
        }
        return potential_cells;
    }
}