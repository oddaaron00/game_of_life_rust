use game_of_life::{Config, Game};
use std::{env, error::Error, process, thread, time};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::build(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let cycle_count = config.get_cycle_count();
    let mut game = Game::new(config);

    game.print_game_state();
    for _ in get_cycle_range(cycle_count) {
        let period = time::Duration::from_millis(100);
        thread::sleep(period);
        game.step();
    }
    Ok(())
}

/// Gets either a [RangeExpr](std::ops::Range) from 0 to `end`, or a [RangeFromExpr](std::ops::RangeFrom) if `end` is `0`
fn get_cycle_range(end: usize) -> Box<dyn Iterator<Item = usize>> {
    if end == 0 {
        Box::new(0..)
    } else {
        Box::new(0..end)
    }
}
