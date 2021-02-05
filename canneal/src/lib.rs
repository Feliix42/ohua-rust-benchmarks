use rand::Rng;

pub mod netlist;

pub enum MoveDecision {
    Good,
    Bad,
    Rejected,
}

pub fn assess_move<R: Rng>(total_cost: f64, temperature: f64, rng: &mut R) -> MoveDecision {
    if total_cost < 0f64 {
        MoveDecision::Good
    } else {
        let random_value: f64 = rng.gen();
        let boltzman = (-total_cost / temperature).exp();
        if boltzman > random_value {
            MoveDecision::Bad
        } else {
            MoveDecision::Rejected
        }
    }
}

/// Check whether the design has convergedor the maximum number of steps was reached
pub fn keep_going(
    completed_temp_steps: i32,
    max_temp_steps: Option<i32>,
    accepted_good_moves: i32,
    accepted_bad_moves: i32,
) -> bool {
    if let Some(bound) = max_temp_steps {
        completed_temp_steps < bound
    } else {
        // TODO: they had a global variable in this as well -> copy?
        accepted_good_moves > accepted_bad_moves
    }
}

pub fn calculate_delta_routing_cost(a: &netlist::NetlistElement, b: &netlist::NetlistElement) -> f64 {
    // TODO: WIP-ish implementation of the original
    let mut delta_cost = a.swap_cost(&a.location, &b.location);
    delta_cost += b.swap_cost(&b.location, &a.location);

    delta_cost
}