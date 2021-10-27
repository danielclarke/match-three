use macroquad::{prelude::*, rand::gen_range};

const GEMS: [Color; 8] = [BLACK, RED, BLUE, GREEN, YELLOW, ORANGE, PINK, PURPLE];

pub struct Board {
    width: i16,
    height: i16,
    num_cells: i16,
    hidden_top_rows: i16,
    cells: Vec<i16>,
    next_gems: Option<[i16; 3]>,
    active_gems: Option<[i16; 3]>,
    spawn_chances: [f32; 3],
    spawn_chance_changes: f32,
}

impl Board {
    pub fn new(width: i16, height: i16) -> Board {
        let board = Board {
            width: width,
            height: height,
            num_cells: width * height,
            hidden_top_rows: 2,
            cells: vec![0; (width * height) as usize],
            active_gems: None,
            next_gems: None,
            spawn_chances: [0.0, 0.2, 0.95],
            spawn_chance_changes: 0.9,
        };

        board
    }

    pub fn idx_xy(&self, idx: i16) -> (i16, i16) {
        let y = (idx / self.width) as i16;
        let x = (idx % self.width) as i16;
        (x, y)
    }

    pub fn xy_idx(&self, x: i16, y: i16) -> usize {
        (y * self.width + x) as usize
    }

    fn drop(&mut self) {
        let mut cells = vec![0; self.num_cells as usize];
        for (idx, cell) in self.cells.iter().enumerate().rev() {
            let (x, y) = self.idx_xy(idx as i16);
            if *cell > 0 {
                let dy = if y < self.height - 1 && cells[self.xy_idx(x, y + 1)] == 0 {
                    1
                } else {
                    0
                };
                cells[self.xy_idx(x, y + dy)] = *cell;
            }
        }
        match self.active_gems {
            None => {}
            Some(mut active_gems) => {
                for i in 0..active_gems.len() {
                    let (x, y) = self.idx_xy(active_gems[i]);
                    let idx_p = self.xy_idx(x, y + 1);
                    active_gems[i] = idx_p as i16;
                }
                self.active_gems = Some(active_gems);
            }
        }
        self.cells = cells;
    }

    pub fn is_static(&self) -> bool {
        for (idx, cell) in self.cells.iter().enumerate() {
            let (x, y) = self.idx_xy(idx as i16);
            if *cell > 0 {
                if y < self.height - 1 && self.cells[self.xy_idx(x, y + 1)] == 0 {
                    return false;
                };
            }
        }
        true
    }

    fn next_match(&self) -> Vec<usize> {
        let mut matching_cells = vec![];
        for (idx, cell) in self.cells.iter().enumerate().rev() {
            if *cell > 0 {
                let (x, y) = self.idx_xy(idx as i16);
                if x > 0
                    && x < self.width - 1
                    && *cell == self.cells[self.xy_idx(x - 1, y)]
                    && *cell == self.cells[self.xy_idx(x + 1, y)]
                {
                    matching_cells.append(&mut vec![
                        idx,
                        self.xy_idx(x - 1, y),
                        self.xy_idx(x + 1, y),
                    ])
                }
                if y > 0
                    && y < self.height - 1
                    && *cell == self.cells[self.xy_idx(x, y - 1)]
                    && *cell == self.cells[self.xy_idx(x, y + 1)]
                {
                    matching_cells.append(&mut vec![
                        idx,
                        self.xy_idx(x, y - 1),
                        self.xy_idx(x, y + 1),
                    ])
                }
                if x > 0
                    && x < self.width - 1
                    && y > 0
                    && y < self.height - 1
                    && *cell == self.cells[self.xy_idx(x - 1, y - 1)]
                    && *cell == self.cells[self.xy_idx(x + 1, y + 1)]
                {
                    matching_cells.append(&mut vec![
                        idx,
                        self.xy_idx(x - 1, y - 1),
                        self.xy_idx(x + 1, y + 1),
                    ])
                }
                if x > 0
                    && x < self.width - 1
                    && y > 0
                    && y < self.height - 1
                    && *cell == self.cells[self.xy_idx(x - 1, y + 1)]
                    && *cell == self.cells[self.xy_idx(x + 1, y - 1)]
                {
                    matching_cells.append(&mut vec![
                        idx,
                        self.xy_idx(x - 1, y + 1),
                        self.xy_idx(x + 1, y - 1),
                    ])
                }
            }
        }
        matching_cells
    }

    fn clear_match(&mut self, matching_cells: Vec<usize>) {
        for idx in matching_cells {
            self.cells[idx] = 0;
        }
    }

    fn spawn_column(&mut self, level: i16) {
        let r = gen_range(0.0f32, 1.0f32);
        let mut next_gems = [0; 3];
        if r > self.spawn_chances[2] * self.spawn_chance_changes.powi(level as i32) {
            next_gems[0] = gen_range(1 as i16, GEMS.len() as i16);
            next_gems[1] = gen_range(1 as i16, GEMS.len() as i16);
            while next_gems[0] == next_gems[1] {
                next_gems[1] = gen_range(1 as i16, GEMS.len() as i16);
            }
            next_gems[2] = gen_range(1 as i16, GEMS.len() as i16);
            while next_gems[0] == next_gems[2] || next_gems[1] == next_gems[2] {
                next_gems[2] = gen_range(1 as i16, GEMS.len() as i16);
            }
        } else if r > self.spawn_chances[1] * self.spawn_chance_changes.powi(level as i32) {
            next_gems[0] = gen_range(1 as i16, GEMS.len() as i16);
            next_gems[1] = next_gems[0];
            next_gems[2] = gen_range(1 as i16, GEMS.len() as i16);
            while next_gems[0] == next_gems[2] {
                next_gems[2] = gen_range(1 as i16, GEMS.len() as i16);
            }
        } else {
            next_gems[0] = gen_range(1 as i16, GEMS.len() as i16);
            next_gems[1] = next_gems[0];
            next_gems[2] = next_gems[0];
        };
        self.next_gems = Some(next_gems);
    }

    fn check_collision(&mut self, dx: i16) -> bool {
        match self.active_gems {
            None => {
                return false;
            }
            Some(active_gems) => {
                for i in 0..active_gems.len() {
                    let (x, y) = self.idx_xy(active_gems[i]);
                    let idx_p = self.xy_idx(x + dx, y);
                    if self.cells[idx_p] != 0 {
                        return true;
                    }
                }
            }
        };
        false
    }

    pub fn handle_input(&mut self, left: bool, right: bool, up: bool) {
        match self.active_gems {
            None => {}
            Some(mut active_gems) => {
                let (x, _y) = self.idx_xy(active_gems[0] as i16);
                let dx = if x > 0 && left {
                    -1
                } else if x < self.width - 1 && right {
                    1
                } else {
                    0
                };
                if dx != 0 && !self.check_collision(dx) {
                    for i in 0..active_gems.len() {
                        let (x, y) = self.idx_xy(active_gems[i]);
                        let idx = self.xy_idx(x, y);
                        let idx_p = self.xy_idx(x + dx, y);
                        self.cells[idx_p] = self.cells[idx];
                        self.cells[idx] = 0;
                        active_gems[i] = idx_p as i16;
                    }
                }
                if up {
                    let g0 = self.cells[active_gems[0] as usize];
                    let g1 = self.cells[active_gems[1] as usize];
                    let g2 = self.cells[active_gems[2] as usize];

                    self.cells[active_gems[0] as usize] = g1;
                    self.cells[active_gems[1] as usize] = g2;
                    self.cells[active_gems[2] as usize] = g0;
                }
                self.active_gems = Some(active_gems);
            }
        }
    }

    fn check_game_over(&mut self) -> bool {
        for i in (self.hidden_top_rows - 1) * self.width
            ..(self.hidden_top_rows - 1) * self.width + self.width
        {
            if self.cells[i as usize] != 0 {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, level: i16) -> i16 {
        let mut cleared_cells = 0;
        if self.is_static() {
            let matching_cells = self.next_match();
            if matching_cells.len() > 0 {
                cleared_cells = matching_cells.len();
                self.clear_match(matching_cells);
                self.active_gems = None;
            } else {
                if !self.check_game_over() {
                    if let Some(gems) = self.next_gems {
                        self.cells[2] = gems[0];
                        self.cells[8] = gems[1];
                        self.cells[14] = gems[2];
                    }
                    self.active_gems = Some([2, 8, 14]);
                    self.spawn_column(level);
                } else {
                    self.active_gems = None;
                }
            }
        } else {
            self.drop();
        }
        cleared_cells as i16
    }

    pub fn render(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let visible_height = self.height - self.hidden_top_rows;
        let ratio = sw / sh;
        let sq_size_x = 320.0 / ratio / visible_height as f32;
        let sq_size_y = 320.0 / visible_height as f32;
        for (idx, cell) in self.cells[(self.width * self.hidden_top_rows) as usize..]
            .iter()
            .enumerate()
        {
            let (x, y) = self.idx_xy(idx as i16);
            draw_rectangle(
                x as f32 * sq_size_x + 160.0 - self.width as f32 * sq_size_x / 2.0,
                y as f32 * sq_size_y,
                sq_size_x - 0.5,
                sq_size_y - 0.5,
                GEMS[*cell as usize],
            );
        }

        let next_gems = match [self.active_gems, self.next_gems] {
            [Some(gems), _] if gems[0] < self.width * self.hidden_top_rows => Some([
                self.cells[gems[0] as usize],
                self.cells[gems[1] as usize],
                self.cells[gems[2] as usize],
            ]),
            [_, Some(gems)] => Some(gems),
            _ => None,
        };

        if let Some(next_gems) = next_gems {
            for (idx, cell) in next_gems.iter().enumerate() {
                draw_rectangle(
                    160.0 - self.width as f32 * sq_size_x / 2.0 - sq_size_x * 2.0,
                    idx as f32 * sq_size_y,
                    sq_size_x - 0.5,
                    sq_size_y - 0.5,
                    GEMS[*cell as usize],
                );
            }
        }
    }
}
