use good_lp::variable::ProblemVariables;
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};
use crate::data::get_item;
use crate::resource::CelestialResource;
use std::collections::HashMap;

#[derive(Default)]
pub struct Value {
    pub lustering_allow: f64,
    pub sheen_compound: f64,
    pub gleaming_alloy: f64,
    pub motley_compound: f64,
    pub precious_alloy: f64,
    pub condensed_alloy: f64,
    pub fiber_composite: f64,
    pub lucent_compound: f64,
    pub opulent_compound: f64,
    pub glossy_compound: f64,
    pub reactive_gas: f64,
    pub noble_gas: f64,
    pub crystal_compound: f64,
    pub dark_compound: f64,
    pub base_metals: f64,
    pub heavy_metals: f64,
    pub toxic_metals: f64,
    pub industrial_fibers: f64,
    pub noble_metals: f64,
    pub reactive_metals: f64,
    pub supertensile_plastics: f64,
    pub polyaramids: f64,
    pub construction_blocks: f64,
    pub nanites: f64,
    pub coolant: f64,
    pub condensates: f64,
    pub silicate_glass: f64,
    pub smartfab_units: f64,
    pub suspended_plasma: f64,
    pub heavy_water: f64,
    pub plasmoids: f64,
    pub liquid_ozone: f64,
    pub ionic_solutions: f64,
    pub oxygen_isotopes: f64,
}

#[derive(Default)]
pub struct ResourceHarvestProblem {
    vars: ProblemVariables,
    value: Value,
    pub available_array: i32,
    days: f64,
    
    total_value: Expression,
    total_array: Expression,
    consumed_key: HashMap<String, Expression>,
    consumed_planet: HashMap<i64, Expression>,
    resource_output: HashMap<i64, Expression>,
    pub available_key: HashMap<String, i32>,
    pub available_planet: HashMap<i64, i32>,
    pub minimum_output: HashMap<i64, f64>,
}

impl ResourceHarvestProblem {
    pub fn new(
        available_key: HashMap<String, i32>,
        available_planet: HashMap<i64, i32>,
        mut minimum_output: HashMap<i64, f64>,
        value: Value,
        days: f64,
    ) -> ResourceHarvestProblem {
        let available_array = available_key.values().copied().sum();
        ResourceHarvestProblem {
            vars: variables!(),
            value,
            available_array,
            days,

            total_value: 0.into(),
            total_array: 0.into(),
            consumed_key: HashMap::new(),
            consumed_planet: HashMap::new(),
            resource_output: HashMap::new(),
            available_key,
            available_planet,
            minimum_output,
        }
    }

    pub fn add_fuel(
        &mut self,
        material_id: i64,
        gj_per_unit: f64,
        gj_needed: f64,
        to_fuel: f64
    ) {
        let quantity = gj_needed / gj_per_unit * 24. * self.days * to_fuel;
        self.minimum_output
            .entry(material_id)
            .and_modify(|value| *value += quantity)
            .or_insert(quantity);
    }

    pub fn add_resource(&mut self, resource: CelestialResource) -> Variable {
        let planet_limit = self.available_planet
            .get(&resource.planet_id)
            .copied()
            .unwrap_or(22);
        let consumed_key = self.consumed_key
            .entry(resource.key.clone())
            .or_insert(0.into());
        let consumed_planet = self.consumed_planet
            .entry(resource.planet_id)
            .or_insert(0.into());
        let resource_output = self.resource_output
            .entry(resource.resource_type_id)
            .or_insert(0.into());
        let array_quantity = self.vars.add(variable().min(0).max(planet_limit));

        self.total_value += get_resource_value(&resource, &self.value) * array_quantity * self.days * 24.;
        self.total_array += array_quantity;
        *consumed_key += array_quantity;
        *consumed_planet += array_quantity;
        *resource_output += array_quantity * resource.init_output * self.days * 24.;

        array_quantity
    }

    pub fn best_production(self) -> impl Solution {
               
        let mut solution = self.vars
            .maximise(self.total_value)
            .using(default_solver)
            .with(self.total_array.eq(self.available_array))
            ;

        for (key, consumed_key) in &self.consumed_key {
            let available_key = self.available_key.get(key).copied().unwrap_or(0);
            solution = solution.with(consumed_key.clone().leq(available_key));
        }

        for (planet_id, consumed_planet) in &self.consumed_planet {
            let available_planet = self.available_planet.get(&planet_id).copied().unwrap_or(0);
            solution = solution.with(consumed_planet.clone().leq(available_planet));
        }
        
        for (resource_type_id, minimum_output) in &self.minimum_output {
            let sum_resource_output: Expression = self.resource_output
                .iter()
                .filter(|(key, _)| *key == resource_type_id)
                .map(|(_, resource_output)| resource_output)
                .sum();
            solution = solution.with(sum_resource_output.geq(*minimum_output));
        }

        solution.solve().unwrap()
    }
}

pub fn get_resource_value(resource: &CelestialResource, value: &Value) -> f64 {
    match resource.resource_type_id {
        42001000000 => value.lustering_allow * resource.init_output,
        42001000001 => value.sheen_compound * resource.init_output,
        42001000002 => value.gleaming_alloy * resource.init_output,
        42001000003 => value.condensed_alloy * resource.init_output,
        42001000004 => value.precious_alloy * resource.init_output,
        42001000005 => value.motley_compound * resource.init_output,
        42001000006 => value.fiber_composite * resource.init_output,
        42001000007 => value.lucent_compound * resource.init_output,
        42001000008 => value.opulent_compound * resource.init_output,
        42001000009 => value.glossy_compound * resource.init_output,
        42001000010 => value.crystal_compound * resource.init_output,
        42001000011 => value.dark_compound * resource.init_output,
        42002000012 => value.heavy_water * resource.init_output,
        42002000013 => value.suspended_plasma * resource.init_output,
        42002000014 => value.liquid_ozone * resource.init_output,
        42002000015 => value.ionic_solutions * resource.init_output,
        42002000016 => value.oxygen_isotopes * resource.init_output,
        42002000017 => value.plasmoids * resource.init_output,
        42001000018 => value.reactive_gas * resource.init_output,
        42001000019 => value.noble_gas * resource.init_output,
        42001000020 => value.base_metals * resource.init_output,
        42001000021 => value.heavy_metals * resource.init_output,
        42001000022 => value.noble_metals * resource.init_output,
        42001000023 => value.reactive_metals * resource.init_output,
        42001000024 => value.toxic_metals * resource.init_output,
        42001000025 => value.industrial_fibers * resource.init_output,
        42001000026 => value.supertensile_plastics * resource.init_output,
        42001000027 => value.polyaramids * resource.init_output,
        42001000028 => value.coolant * resource.init_output,
        42001000029 => value.condensates * resource.init_output,
        42001000030 => value.construction_blocks * resource.init_output,
        42001000031 => value.nanites * resource.init_output,
        42001000032 => value.silicate_glass * resource.init_output,
        42001000033 => value.smartfab_units * resource.init_output,
        _ => panic!("Invalid attribute name: {}", get_item(resource.resource_type_id).unwrap().en_name),
    }
}

