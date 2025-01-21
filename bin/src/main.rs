use {clap::Parser, std::fs};

mod cli;

const BASE_PATH: &str = env!("CARGO_MANIFEST_DIR");
const LATEST_DAY: usize = 9;
const LATEST_YEAR: usize = 2024;

fn main() {
    let args = cli::Cli::parse();
    let year = args.year.unwrap_or(LATEST_YEAR);
    let day = args.day.unwrap_or(LATEST_DAY);

    get_solution(year, day);
}

fn get_solution(year: usize, day: usize) {
    let filename = format!("{BASE_PATH}/res/{year}/day{day}.txt");
    let data = fs::read_to_string(&filename).unwrap_or_else(|e| {
        panic!("Failed to load {filename}: {e:?}");
    });
    match year {
        2023 => aoc_2023::run(day, &data),
        2024 => aoc_2024::run(day, &data),
        other => panic!("Unknown year {other}"),
    }
}
