use rand::Rng;
use std::f64::consts::E;

#[derive(Clone)]
struct Parameters {
    max_displacement: f64,
    temperature: f64,
}

#[derive(Clone)]
struct MCStructure<E>
where
    E: Fn(f64) -> f64 + std::clone::Clone,
{
    x: f64,
    energy: f64,
    energy_function: E,
    params: Parameters,
}

struct MCModel {
    average_energy: f64,
    target_energy: f64,
    acceptance_rate: f64,
    steps: usize,
}

struct MCModelVec {
    result_vec: Vec<MCModel>,
}

impl MCModelVec {
    fn new() -> Self {
        let new_vec: Vec<MCModel> = Vec::new();
        MCModelVec {
            result_vec: new_vec,
        }
    }
}

fn main() {
    let mut energy_vec: Vec<f64> = Vec::new();
    let mut target_energy_vec: Vec<f64> = Vec::new();
    let mut acceptance_vec: Vec<f64> = Vec::new();
    let mut step_vec: Vec<usize> = Vec::new();

    let mut results = MCModelVec::new();

    let kc = 0.1;
    let potential_energy = |x: f64| kc * x.powi(2);
    let x = 100_f64;
    // Equation is e^-(ΔE/kB*T)
    // When kB = 1 -> P(o->n) = e^-(ΔE/T)
    for i in 1..=10 {
        let t = i as f64 / 10_f64;
        let target = t / 2_f64;
        let es = 0.1;
        let energy = potential_energy(x);
        let params = Parameters {
            max_displacement: 10_f64,
            temperature: t,
        };
        let structure = MCStructure {
            x,
            energy,
            energy_function: potential_energy,
            params,
        };

        let new_model = monte_carlo(structure, target, es);
        energy_vec.push(new_model.average_energy);
        target_energy_vec.push(new_model.target_energy);
        acceptance_vec.push(new_model.acceptance_rate);
        step_vec.push(new_model.steps);

        results.result_vec.push(new_model);
    }

    println!("Approximate Energies:\n{energy_vec:?}\n");
    println!("Target Energies:\n{target_energy_vec:?}\n");
    println!("Acceptance Rates:\n{acceptance_vec:?}\n");
    println!("Number of Steps:\n{step_vec:?}\n");
}

// Returns (Approximate Average Energy, Acceptance Rate, Number of MC Steps)
fn monte_carlo<E>(structure: MCStructure<E>, target: f64, es: f64) -> MCModel
where
    E: Fn(f64) -> f64 + std::clone::Clone,
{
    let mut steps: usize = 0;
    let mut total_energy = 0_f64;
    let mut accepted: usize = 0;

    let lower = target - target * es;
    let upper = target + target * es;
    let mut curr_structure = structure;
    let mut running_average_energy = 0_f64;
    loop {
        let acc_new_state;
        if lower < running_average_energy && running_average_energy < upper {
            break;
        }
        (curr_structure, acc_new_state) = metropolis(curr_structure);

        if acc_new_state {
            accepted += 1;
        }
        steps += 1;
        total_energy += curr_structure.energy;
        running_average_energy = total_energy / steps as f64;
    }

    let average_energy = running_average_energy;
    let acceptance_rate = accepted as f64 / steps as f64;
    MCModel {
        average_energy,
        target_energy: target,
        acceptance_rate,
        steps,
    }
}

fn metropolis<E>(structure: MCStructure<E>) -> (MCStructure<E>, bool)
where
    E: Fn(f64) -> f64 + std::clone::Clone,
{
    let mut accepted = false;

    let mut rng = rand::rng(); // Generate a uniformly distributed range
    let rand_x = rng.random_range(0.0..=1.0); // get one sample between 0 and 1 inclusive
    //
    let displacement = structure.params.max_displacement * (rand_x - 0.5);
    let x_new = structure.x + displacement;
    let e_new = (structure.energy_function)(x_new);
    let new_structure = {
        let poss_new = MCStructure {
            x: x_new,
            energy: e_new,
            energy_function: structure.energy_function.clone(),
            params: structure.params.clone(),
        };

        if e_new <= structure.energy {
            accepted = true;
            poss_new
        } else {
            let mut rng = rand::rng();
            let rand_val = rng.random_range(0.0..=1.0);
            if rand_val < E.powf(-(e_new - structure.energy) / structure.params.temperature) {
                accepted = true;
                poss_new
            } else {
                structure
            }
        }
    };
    (new_structure, accepted)
}
