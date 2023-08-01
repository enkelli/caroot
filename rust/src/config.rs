use hocon::{Error, HoconLoader};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub generations: i32,
    pub population_size: i32,
    pub radius: i32,
    pub state_count: i32,
    pub max_dev_iters: i32,
    pub mutation_perc: i32,
    pub output_file: String,
}

pub fn get_config() -> Config {
    let default_conf = Config {
        generations: 1000000,
        population_size: 8,
        radius: 1,
        state_count: 4,
        max_dev_iters: 100,
        mutation_perc: 10,
        output_file: "ruleset.csv".to_string(),
    };

    load_config().unwrap_or(default_conf)
}

fn load_config() -> Result<Config, Error> {
    let path = get_config_file_path();
    HoconLoader::new().load_file(path)?.resolve()
}

fn get_config_file_path() -> String {
    "config.conf".to_string()
}
