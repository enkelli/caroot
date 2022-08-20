use crate::ruleset::RuleSet;

pub struct SqrtCA {
    value: usize,
    target: f64,
    rule_radius: usize,
    pub cells: Vec<u32>,
    prev_cells: Vec<u32>,
}

impl SqrtCA {
    pub fn init(value: usize, rule_radius: i32) -> Self {
        let mut ca = SqrtCA {
            value: value,
            target: (value as f64).sqrt(),
            rule_radius: rule_radius.try_into().unwrap(),
            cells: vec![],
            prev_cells: vec![],
        };
        ca.reset();
        ca
    }

    pub fn reset(&mut self) {
        let border_len = 2 * self.rule_radius + 1;
        let cell_count = self.value + border_len * 2;

        self.cells = (0..cell_count)
            .map(|i| {
                if i < border_len || i >= (cell_count - border_len) {
                    0
                } else {
                    1
                }
            })
            .collect();
        self.prev_cells = self.cells.clone();
    }

    pub fn develop(&mut self, ruleset: &RuleSet, max_dev_iters: i32) {
        for _i in 1..max_dev_iters {
            if !self.evolve(ruleset) {
                return;
            }
        }
    }

    pub fn evolve(&mut self, ruleset: &RuleSet) -> bool {
        let mut changed = false;
        for (pos, _e) in self.cells.iter().enumerate() {
            if pos >= self.rule_radius && pos < (self.cells.len() - self.rule_radius - 1) {
                self.prev_cells[pos] =
                    ruleset.state_for(&self.cells[pos - 1..pos + self.rule_radius + 1]);
                if !changed && self.prev_cells[pos] != self.cells[pos] {
                    changed = true;
                }
            }
        }
        std::mem::swap(&mut self.prev_cells, &mut self.cells);

        changed
    }

    pub fn fitness(&self) -> u32 {
        let mut max_f = 0;
        let mut seq = false;
        let mut after = false;
        let mut redund = 0;

        if self.cells.len() == 0 {
            return 100;
        }

        if self.cells[0] != 0 {
            redund += 1;
        }
        if self.cells[self.cells.len() - 1] != 0 {
            redund += 1;
        }
        if self.prev_cells[self.cells.len() - 1] != 0 {
            redund += 1;
        }

        for (i, val) in self.cells[1..self.cells.len() - 1].iter().enumerate() {
            if self.cells[i] != self.prev_cells[i] {
                redund += 1;
            }
            if after {
                if *val != 0 {
                    redund += 1;
                }
            } else {
                if *val == 0 && seq {
                    seq = false;
                    after = true;
                } else if *val != 0 && (i == (self.cells.len() - 1)) {
                    max_f += 1;
                } else if *val == 0 {
                    continue;
                } else {
                    seq = true;
                    max_f += 1;
                }
            }
        }

        let target = self.target as u32;
        let result = if max_f > target {
            max_f - target
        } else {
            target - max_f
        };

        result + redund
    }
}
