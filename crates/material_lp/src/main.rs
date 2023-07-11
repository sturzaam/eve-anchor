use clap::Parser;
use std::error::Error;
use tabled::{Table, settings::Style};
use anyhow::{Context, Result};
use good_lp::solvers::Solution;

use material_lp::{
    map_objective, parse_decomposed_list, 
    data::{CelestialResource},
    solver::{ResourceHarvestProblem, solution_table}
};



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'D')]
    days: f64,
    #[arg(short = 'C', value_parser = parse_key_val::<String, i32>)]
    constellations: Vec<(String, i32)>,
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

fn main() -> Result<()> {
    let args = Cli::parse();
    // println!("{args:?}");
    let constellations: Vec<(String, i32)> = args.constellations.into_iter().collect();

    let materials = parse_decomposed_list().with_context(|| "Failed to parse decomposed list.")?;
    let available_outposts = constellations.iter().map(|(_, value)| value).sum();
    let (available_constellation, available_planet, minimum_output, celestial_resources, value) = map_objective(materials, constellations);
    let mut harvest = ResourceHarvestProblem::new(
        available_constellation,
        available_planet,
        minimum_output,
        value,
        available_outposts,
        args.days,
    );
    let variables: Vec<_> = celestial_resources
        .clone()
        .into_iter()
        .map(|r| harvest.add_resource(r))
        .collect();
    let solution = harvest.best_production();
    let resource_quantities: Vec<_> = variables.iter().map(|&v| solution.value(v)).collect();
    let celestial_resource_values: Vec<(&CelestialResource, f64)> = celestial_resources
        .iter()
        .zip(resource_quantities)
        .collect();

    let solution_table = solution_table(celestial_resource_values);
    let table = Table::new(solution_table).with(Style::ascii_rounded()).to_string();
    println!("{}", table);
    Ok(())
}
