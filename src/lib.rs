use std::{
    fmt::{self, Display},
    num::ParseIntError,
};

pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub enum ConfigError {
    ArgsParsingError(ParseIntError),
    StartingSizeMissing,
    CycleCountMissing,
    TooFewStartingPoints,
    StartingPointsParsingError,
}

impl std::error::Error for ConfigError {}

impl From<ParseIntError> for ConfigError {
    fn from(err: ParseIntError) -> Self {
        ConfigError::ArgsParsingError(err)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ArgsParsingError(parse_int_error) => {
                write!(f, "Could not parse arguments: {parse_int_error}")
            }
            ConfigError::StartingSizeMissing => write!(f, "No starting size provided."),
            ConfigError::CycleCountMissing => write!(f, "No cycle count provided."),
            ConfigError::TooFewStartingPoints => write!(f, "Not enough starting points provided."),
            ConfigError::StartingPointsParsingError => write!(f, "Cannot parse starting points"),
        }
    }
}

/// A struct representing the Game of Life game state.
pub struct Game {
    grid: Grid,
}

impl Game {
    /// Initialise a new game.
    ///
    /// # Arguments
    /// * `config` - A [Config] object
    pub fn new(config: Config) -> Self {
        let x = config.get_x();
        let y = config.get_y();
        let starting_cells = config.get_starting_cells();

        Self {
            grid: Grid::new(x, y, starting_cells),
        }
    }

    /// Runs one game cycle.
    fn step_forward(&mut self) {
        self.grid.step_forward();
    }

    /// Prints the current game state to the console.
    pub fn print_game_state(&self) {
        self.grid.print_grid();
    }

    /// Runs one cycle and prints the new game state to the console.
    pub fn step(&mut self) {
        self.step_forward();
        self.print_game_state();
    }
}

pub struct Config {
    grid_width: u8,
    grid_height: u8,
    /// The number of cycles to complete, or `0` to run indefinitely
    cycle_count: usize,
    /// A vector of coordinates of cells which should start in an alive state
    starting_cells: Vec<(u8, u8)>,
}

impl Config {
    /// Builds and returns the [Config] object, or a [ConfigError].
    ///
    /// # Arguments
    ///
    /// * `args` - A vector with a minimum length of five containing the config arguments:
    ///     * `0` - Grid width, in cells
    ///     * `1` - Grid height, in cells
    ///     * `2` - Cycle count, or `0` to run indefinitely
    ///     * `3+` - At least three starting coordinates, each in the format `x,y`, where `0,0` is the bottom-left cell
    ///             (this is the minimum number of cells required to create a sustained game)
    ///
    ///
    /// # Example
    /// ```
    /// use game_of_life::config::Config;
    ///
    /// let args = vec![String::from("10"), String::from("10"), String::from("2,4"), String::from("2,5"), String::from("3,5")];
    /// let config = Config::build(args);
    /// ```
    pub fn build(args: Vec<String>) -> ConfigResult<Self> {
        if args.len() < 2 {
            return Err(ConfigError::StartingSizeMissing);
        } else if args.len() < 3 {
            return Err(ConfigError::CycleCountMissing);
        } else if args.len() < 6 {
            return Err(ConfigError::TooFewStartingPoints);
        }

        let grid_width: u8 = args[0].parse()?;
        let grid_height: u8 = args[1].parse()?;
        let cycle_count: usize = args[2].parse()?;
        let mut starting_cells: Vec<(u8, u8)> = Vec::new();

        let starting_cells_strings = &args[3..];

        for cell_string in starting_cells_strings {
            let components: Vec<&str> = cell_string.split(',').collect();

            if components.len() != 2 {
                return Err(ConfigError::StartingPointsParsingError);
            }

            let x_component = match components[0].parse::<u8>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(ConfigError::StartingPointsParsingError);
                }
            };
            let y_component = match components[1].parse::<u8>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(ConfigError::StartingPointsParsingError);
                }
            };

            let point = (x_component, y_component);

            if starting_cells.contains(&point) {
                continue;
            } else {
                starting_cells.push((x_component, y_component));
            }
        }

        Ok(Self {
            grid_width,
            grid_height,
            cycle_count,
            starting_cells,
        })
    }

    pub fn get_x(&self) -> u8 {
        self.grid_width
    }

    pub fn get_y(&self) -> u8 {
        self.grid_height
    }

    pub fn get_starting_cells(&self) -> Vec<(u8, u8)> {
        self.starting_cells.clone()
    }

    pub fn get_cycle_count(&self) -> usize {
        self.cycle_count
    }
}

#[derive(Clone)]
pub struct Grid {
    /// A one-dimensional vector of [Cells](Cell) representing the flattened grid.
    grid: Vec<Cell>,
    width: u8,
    height: u8,
}

impl Grid {
    pub fn new(width: u8, height: u8, starting_cells: Vec<(u8, u8)>) -> Self {
        let mut grid: Vec<Cell> = Vec::new();

        for b in 0..height {
            for a in 0..width {
                let cell = Cell::new(a, b, starting_cells.contains(&(a, b)));
                grid.push(cell);
            }
        }

        Self {
            grid,
            width,
            height,
        }
    }

    /// Returns the cell at the corresponding coordinates or `None` if the coordinates point outside the grid.
    fn get_cell(&self, x: i16, y: i16) -> Option<&Cell> {
        if x < 0 || x >= self.width as i16 || y < 0 || y >= self.height as i16 {
            None
        } else {
            Some(&self.grid[(y * self.width as i16 + x) as usize])
        }
    }

    /// Updates each cell in the grid according to Conway's rules
    pub fn step_forward(&mut self) {
        let initial_grid_state = self.clone();

        for cell in &mut self.grid {
            let (x, y) = cell.get_coords();
            let (x, y) = (x as i16, y as i16);

            let mut neighbours = Vec::new();

            for i in vec![x - 1, x, x + 1] {
                for j in vec![y - 1, y, y + 1] {
                    if i == x && j == y {
                        continue;
                    }
                    neighbours.push(initial_grid_state.get_cell(i, j));
                }
            }

            cell.update_state(neighbours);
        }
    }

    /// Prints the grid in-place in the console.
    pub fn print_grid(&self) {
        let mut line: Vec<&Cell> = Vec::new();

        std::process::Command::new("clear")
            .status()
            .expect("Could not run command \"clear\"");

        for i in (0..(self.width * self.height)).rev() {
            if i > self.width * self.height - self.width - 1 {
                line.insert(0, &self.grid[i as usize])
            } else {
                line[(i % self.width) as usize] = &self.grid[i as usize];
            }
            if i % self.width == 0 {
                for &e in &line {
                    print!("{:?} ", e);
                }
                println!();
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum State {
    Alive = 1,
    Dead = 0,
}

#[derive(Clone)]
pub struct Cell {
    state: State,
    x: u8,
    y: u8,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.state.clone() == State::Alive {
                'O'
            } else {
                '.'
            }
        )
    }
}

impl Cell {
    pub fn new(x: u8, y: u8, alive: bool) -> Self {
        Self {
            state: if alive { State::Alive } else { State::Dead },
            x,
            y,
        }
    }

    pub fn update_state(&mut self, neighbours: Vec<Option<&Cell>>) {
        self.state = calc_new_state(self.state.clone(), neighbours);
    }

    pub fn get_coords(&self) -> (u8, u8) {
        (self.x, self.y)
    }
}

fn calc_new_state(current_state: State, neighbours: Vec<Option<&Cell>>) -> State {
    let live_neighbour_count: u8 = neighbours
        .iter()
        .map(|&cell| unwrap_cell_state_value(cell))
        .reduce(|acc, curr| acc + curr)
        .unwrap_or_default();

    #[allow(clippy::if_same_then_else)]
    if current_state == State::Alive && (live_neighbour_count == 2 || live_neighbour_count == 3) {
        State::Alive
    } else if current_state == State::Dead && live_neighbour_count == 3 {
        State::Alive
    } else {
        State::Dead
    }
}

fn unwrap_cell_state_value(cell_option: Option<&Cell>) -> u8 {
    match cell_option {
        Some(cell) => cell.state.clone() as u8,
        None => 0,
    }
}
