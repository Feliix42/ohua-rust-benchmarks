use rand::Rng;
#[cfg(feature = "transactional")]
use stm::{StmResult, TVar, Transaction};

#[cfg(all(not(feature = "transactional"), not(feature = "ohua")))]
pub mod netlist;

#[cfg(feature = "ohua")]
pub mod ohua_netlist;
#[cfg(feature = "transactional")]
pub mod stm_netlist;

pub enum MoveDecision {
    Good,
    Bad,
    Rejected,
}

#[cfg(not(feature = "transactional"))]
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

/// Check whether the design has converged or the maximum number of steps was reached
#[cfg(not(feature = "transactional"))]
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

#[cfg(all(not(feature = "transactional"), not(feature = "ohua")))]
pub fn calculate_delta_routing_cost(
    a: &netlist::NetlistElement,
    b: &netlist::NetlistElement,
) -> f64 {
    // TODO: WIP-ish implementation of the original
    let mut delta_cost = a.swap_cost(&a.location, &b.location);
    delta_cost += b.swap_cost(&b.location, &a.location);

    delta_cost
}

#[cfg(feature = "transactional")]
pub fn assess_move(total_cost: f64, temperature: f64, random_value: f64) -> MoveDecision {
    if total_cost < 0f64 {
        MoveDecision::Good
    } else {
        let boltzman = (-total_cost / temperature).exp();
        if boltzman > random_value {
            MoveDecision::Bad
        } else {
            MoveDecision::Rejected
        }
    }
}

/// Check whether the design has converged or the maximum number of steps was reached
#[cfg(feature = "transactional")]
pub fn keep_going(
    completed_temp_steps: i32,
    max_temp_steps: Option<i32>,
    accepted_good_moves: &TVar<i32>,
    accepted_bad_moves: &TVar<i32>,
) -> bool {
    if let Some(bound) = max_temp_steps {
        completed_temp_steps < bound
    } else {
        // TODO: they had a global variable in this as well -> copy?
        accepted_good_moves.read_atomic() > accepted_bad_moves.read_atomic()
    }
}

#[cfg(feature = "transactional")]
pub fn calculate_delta_routing_cost(
    a: &stm_netlist::NetlistElement,
    b: &stm_netlist::NetlistElement,
    trans: &mut Transaction,
) -> StmResult<f64> {
    // TODO: WIP-ish implementation of the original
    let mut delta_cost = a.swap_cost(&a.location, &b.location, trans)?;
    delta_cost += b.swap_cost(&b.location, &a.location, trans)?;

    Ok(delta_cost)
}
