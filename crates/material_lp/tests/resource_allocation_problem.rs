use std::collections::HashMap;
use good_lp::variable::ProblemVariables;
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};
use float_eq::assert_float_eq;

struct Product {
    city_id: i64,
    factory_id: i64,
    product_id: i64,
    value: f64,
}

struct ResourceAllocationProblem {
    vars: ProblemVariables,
    available_production: f64,
    total_value: Expression,
    total_produced: Expression,
    consumed_city: HashMap<i64, Expression>,
    consumed_factory: HashMap<i64, Expression>,
    product_value: HashMap<i64, Expression>,
    available_city: HashMap<i64, f64>,
    available_factory: HashMap<i64, f64>,
    minimum_value: HashMap<i64, f64>,
}

impl ResourceAllocationProblem {
    fn new(available_production: f64) -> ResourceAllocationProblem {
        
        ResourceAllocationProblem { vars: variables!(),
            available_production,
            total_value: 0.into(),
            total_produced: 0.into(),
            consumed_city: HashMap::new(),
            consumed_factory: HashMap::new(),
            product_value: HashMap::new(),
            available_city: HashMap::new(),
            available_factory: HashMap::new(),
            minimum_value: HashMap::new(),
        }
    }

    fn add(&mut self, product: Product) -> Variable {
        let factory_limit = self.available_factory
            .get(&product.factory_id)
            .copied()
            .unwrap_or(22.);
        let consumed_city = self.consumed_city
            .entry(product.city_id)
            .or_insert(0.into());
        let consumed_factory = self.consumed_factory
            .entry(product.factory_id)
            .or_insert(0.into());
        let product_value = self.product_value
            .entry(product.product_id)
            .or_insert(0.into());
        let amount_to_produce = self.vars
            .add(variable().min(0).max(factory_limit));
            
        self.total_value += amount_to_produce * product.value;
        self.total_produced += amount_to_produce;
        *consumed_city += amount_to_produce;
        *consumed_factory += amount_to_produce;
        *product_value += amount_to_produce * product.value;

        amount_to_produce
    }

    fn best_product_quantities(self) -> impl Solution {
        let mut solution = self.vars
            .maximise(self.total_value)
            .using(default_solver)
            .with(self.total_produced.eq(self.available_production))
            ;

        for (city_id, consumed_city) in &self.consumed_city {
            let available_city = self.available_city.get(&city_id).copied().unwrap_or(0.);
            solution = solution.with(consumed_city.clone().leq(available_city));
        }

        for (factory_id, consumed_factory) in &self.consumed_factory {
            let available_factory = self.available_factory.get(&factory_id).copied().unwrap_or(0.);
            solution = solution.with(consumed_factory.clone().leq(available_factory));
        }

        
        for (product_id, minimum_value) in &self.minimum_value {
            let sum_product_value: Expression = self.product_value
                .iter()
                .filter(|(key, _)| *key == product_id)
                .map(|(_, product_value)| product_value)
                .sum();
            solution = solution.with(sum_product_value.geq(*minimum_value));
        }

        solution.solve().unwrap()
    }
}

#[test]
fn test_product_quantities() {
    let products = vec![

        // City 1

        Product { city_id: 1, factory_id: 1, product_id: 1, value: 1.},
        Product { city_id: 1, factory_id: 1, product_id: 2, value: 2.},
        Product { city_id: 1, factory_id: 2, product_id: 2, value: 4.},
        Product { city_id: 1, factory_id: 2, product_id: 3, value: 3.},
        Product { city_id: 1, factory_id: 3, product_id: 1, value: 1.},
        Product { city_id: 1, factory_id: 3, product_id: 2, value: 2.},
        Product { city_id: 1, factory_id: 3, product_id: 3, value: 9.},
        Product { city_id: 1, factory_id: 4, product_id: 1, value: 1.},
        Product { city_id: 1, factory_id: 4, product_id: 2, value: 1.},
        Product { city_id: 1, factory_id: 4, product_id: 3, value: 1.},

        // City 2

        Product { city_id: 2, factory_id: 5, product_id: 1, value: 2.},
        Product { city_id: 2, factory_id: 5, product_id: 2, value: 4.},
        Product { city_id: 2, factory_id: 6, product_id: 2, value: 4.},
        Product { city_id: 2, factory_id: 6, product_id: 3, value: 6.},
        Product { city_id: 2, factory_id: 7, product_id: 1, value: 2.},
        Product { city_id: 2, factory_id: 7, product_id: 2, value: 4.},
        Product { city_id: 2, factory_id: 7, product_id: 3, value: 6.},
        Product { city_id: 2, factory_id: 8, product_id: 1, value: 2.},
        Product { city_id: 2, factory_id: 8, product_id: 2, value: 4.},
        Product { city_id: 2, factory_id: 8, product_id: 3, value: 6.},
    ];

    let mut problem = ResourceAllocationProblem::new(60.);
    problem.minimum_value = vec![(1, 11.), (2, 44.)].into_iter().collect();
    problem.available_city = vec![(1, 30.), (2, 30.)].into_iter().collect();
    problem.available_factory = (1..=8).map(|n| (n, 10.)).into_iter().collect();

    let variables: Vec<_> = products
        .into_iter()
        .map(|p| problem.add(p))
        .collect();
    let solution = problem.best_product_quantities();
    let product_quantities: Vec<_> = variables.iter().map(|&v| solution.value(v)).collect();

    println!("{:?}", product_quantities);
    assert_float_eq!(8., product_quantities[0], abs <= 1e-1);
    assert_float_eq!(2., product_quantities[1], abs <= 1e-1);
    assert_float_eq!(10., product_quantities[2], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[3], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[4], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[5], abs <= 1e-1);
    assert_float_eq!(10., product_quantities[6], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[7], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[8], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[9], abs <= 1e-1);
    // city id 2
    assert_float_eq!(1.5, product_quantities[10], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[11], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[12], abs <= 1e-1);
    assert_float_eq!(8.5, product_quantities[13], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[14], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[15], abs <= 1e-1);
    assert_float_eq!(10., product_quantities[16], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[17], abs <= 1e-1);
    assert_float_eq!(0., product_quantities[18], abs <= 1e-1);
    assert_float_eq!(10., product_quantities[19], abs <= 1e-1);
}
