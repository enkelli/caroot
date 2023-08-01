use itertools::Itertools;
use rand::distributions::Uniform;
use rand::Rng;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::Write;

#[derive(Clone)]
pub struct RuleSet {
    rules: BTreeMap<Vec<u32>, u32>,
    state_count: u32,
}

impl RuleSet {
    pub fn random(radius: usize, state_count: u32) -> Self {
        let mut rules: BTreeMap<Vec<u32>, u32> = BTreeMap::new();
        let rule_length = 2 * radius + 1;
        let items: Vec<u32> = (0..state_count)
            .flat_map(|v| vec![v; rule_length])
            .collect();

        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, state_count);
        let range_100 = Uniform::new(1, 100);

        for perm in items.iter().copied().permutations(rule_length).unique() {
            let value = if rng.sample(&range_100) > 50 {
                get_neutral_state(&perm)
            } else {
                rng.sample(&range)
            };
            rules.insert(perm, value);
        }

        RuleSet {
            rules: rules,
            state_count: state_count,
        }
    }

    pub fn mutated(&self, mutation_perc: usize) -> Self {
        let mut new = self.clone();
        new.mutate(mutation_perc);
        new
    }
    pub fn state_for(&self, states: &[u32]) -> u32 {
        self.rules.get(states).copied().unwrap()
    }

    fn mutate(&mut self, mutation_perc: usize) {
        let rule_count_to_mutate = mutation_perc * 100 / self.rules.len();
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, self.rules.len() as u32);
        let mut i: u32 = 0;
        let mutate_indexes: HashSet<u32> = (1..=rule_count_to_mutate)
            .map(|_| rng.sample(range))
            .collect();
        let neutralize_indexes: HashSet<u32> = (1..=rule_count_to_mutate)
            .map(|_| rng.sample(range))
            .collect();
        let states_range = Uniform::new(0, self.state_count);
        for (states, value) in self.rules.iter_mut() {
            if mutate_indexes.contains(&i) {
                *value = rng.sample(states_range);
            }
            if neutralize_indexes.contains(&1) {
                *value = get_neutral_state(states)
            }
            i += 1;
        }
    }

    pub fn to_file(&self, file_path: &str) {
        let mut file = File::create(file_path).expect("Unable to create file.");
        for (states, new_state) in &self.rules {
            let states_str = states
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");

            let content = format!("{},{}\n", states_str, new_state);
            file.write_all(content.as_bytes())
                .expect("Unable to write to file.");
        }
    }
}

impl fmt::Debug for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (states, new_state) in &self.rules {
            write!(f, "{:?} -> {}\n", states, new_state)?;
        }
        write!(f, "")
    }
}

fn get_neutral_state(states: &[u32]) -> u32 {
    states[states.len() / 2]
}
