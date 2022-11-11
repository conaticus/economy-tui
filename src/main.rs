use std::{io::{stdin, stdout, Write}, error::Error, collections::HashMap};
use rand::Rng;

enum CommandCategory {
    General,
    Taxes,
}

use CommandCategory::*;

impl Default for CommandCategory {
    fn default() -> Self {
        Self::General
    }
}

#[derive(Default)]
struct CommandMeta<'a> {
    name: &'a str,
    parameters: Vec<&'a str>,
    description: &'a str,
    category: CommandCategory,
}

const TAX_BRACKETS: [f64; 7] = [10., 12., 22., 24., 32., 35., 37.];

struct GameState {
    tax_rate: f64,
}

impl Default for GameState {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            tax_rate: TAX_BRACKETS[rng.gen_range(0..TAX_BRACKETS.len())],
        }
    }
}

fn get_commands_meta<'a>() -> Vec<CommandMeta<'a>> {
    vec![
        CommandMeta {
            name: "help",
            description: "Display help menu.",
            category: General,
            ..CommandMeta::default()
        },
        CommandMeta {
            name: "exit",
            description: "Exit the program.",
            category: General,
            ..CommandMeta::default()
        },
        CommandMeta {
            name: "tax",
            description: "Get the current tax %.",
            category: Taxes,
            ..CommandMeta::default()
        },
        CommandMeta {
            name: "taxset",
            description: "Set the tax %.",
            category: Taxes,
            parameters: vec!["percentage"]
        }
    ]
}

fn get_command() -> std::io::Result<(String, Vec<String>)> {
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let mut args: Vec<String> = input.split(' ').map(|s| 
        String::from(s).trim().to_lowercase()
    ).collect();

    Ok((String::from(args[0]), args))
}

fn display_help(commands: &Vec<CommandMeta>) {
    let mut categories: HashMap<String, Vec<CommandMeta>> = HashMap::new();
    for meta in commands {
        categories.insert(meta.category.into(), vec![*meta]);
    }
}

fn check_args(parameters: Vec<&str>, args: &Vec<String>) {
    if parameters.len() > args.len() {
        println!("Invalid arguments, expected: {:?}.", parameters);
    }
}

fn parse_arg<T>(arg: &String) -> std::io::Result<T> {
    arg.parse::<T>()?
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut game_state = GameState::default();
    let commands = get_commands_meta();

    loop {
        print!("> ");
        let (command, args) = get_command()?;

        match command.as_str() {
            "help" => display_help(&commands),
            "exit" => break,
            "tax" => println!("{}", game_state.tax_rate),
            "taxset" => {
                check_args(vec!["new_tax"], &args);

                game_state.tax_rate = args[0].parse::<f64>()?;
            },
            _ => continue,
        }
    }

    Ok(())
}