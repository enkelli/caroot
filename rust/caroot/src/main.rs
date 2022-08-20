mod ca;
mod config;
mod ruleset;

use ca::SqrtCA;
use config::get_config;
use ruleset::RuleSet;

fn main() {
    let config = get_config();
    println!("{:#?}\n", config);

    let generations = config.generations;
    let population_size = config.population_size;
    let radius = config.radius;
    let state_count = config.state_count;
    let max_dev_iters = config.max_dev_iters;
    let mutation_perc: usize = config.mutation_perc.try_into().unwrap();

    let mut cas: Vec<SqrtCA> = vec![
        SqrtCA::init(9, radius),
        SqrtCA::init(16, radius),
        SqrtCA::init(25, radius),
        SqrtCA::init(36, radius),
    ];

    let mut best_fitness = std::u32::MAX;

    let mut rulesets: Vec<RuleSet> = (0..population_size)
        .map(|_| RuleSet::random(radius.try_into().unwrap(), state_count.try_into().unwrap()))
        .collect();
    let mut best_ruleset = rulesets[0].clone();

    for gen in 1..=generations {
        let mut local_best_fitness = std::u32::MAX;
        let mut local_best_ruleset = rulesets[0].clone();
        for ruleset in &rulesets {
            let mut curr_fitness = 0;
            for ca in &mut cas {
                ca.reset();
                ca.develop(&ruleset, max_dev_iters);
                curr_fitness += ca.fitness();
            }

            if curr_fitness <= best_fitness {
                best_fitness = curr_fitness;
                best_ruleset = ruleset.clone();
                println!("Gen: {} fitness: {}", gen, best_fitness);
            }
            if curr_fitness <= local_best_fitness {
                local_best_fitness = curr_fitness;
                local_best_ruleset = ruleset.clone();
            }

            if best_fitness == 0 {
                break;
            }
        }

        if best_fitness == 0 {
            break;
        }

        if gen & 4095 == 4095 {
            println!("Gen: {} fitness: {}", gen, local_best_fitness);
        }

        rulesets.clear();
        for _ in 1..=population_size {
            rulesets.push(local_best_ruleset.mutated(mutation_perc))
        }
    }
    println!(
        "Best ruleset (fitness: {}):\n{:?}",
        best_fitness, best_ruleset
    );
}
