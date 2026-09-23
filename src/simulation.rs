use macroquad::prelude::Vec2;

use crate::{
    neural::{Genome, cross_genomes, random_genome},
    physics::Physics,
    racer::{Racer, init_racer, restart_racer},
};

const GENERATION_SIZE: usize = 10;
const ELITE_COUNT: usize = 2;
const CROSSOVER_PARENT_COUNT: usize = 4;
const RANDOM_COUNT: usize = 4;

pub struct SimulationRacer {
    pub racer: Racer,
    pub spawn_position: Vec2,
}

pub struct Simulation {
    pub racers: Vec<SimulationRacer>,
}

pub fn init_simulation(physics: &mut Physics, spawn_positions: &[Vec2]) -> Simulation {
    let racers = spawn_positions
        .iter()
        .copied()
        .map(|spawn_position| SimulationRacer {
            racer: init_racer(physics, spawn_position),
            spawn_position,
        })
        .collect();

    Simulation { racers }
}

/// Builds a ten-genome generation from genomes ranked best to worst.
///
/// The best two survive unchanged, the next four are paired to produce four
/// children, and the remaining four genomes are generated randomly.
pub fn sample_next_generation(ranked_genomes: &[Genome]) -> Vec<Genome> {
    assert_eq!(
        ranked_genomes.len(),
        GENERATION_SIZE,
        "sample_next_generation expects exactly ten ranked genomes"
    );

    let mut next_generation = Vec::with_capacity(GENERATION_SIZE);

    next_generation.extend(ranked_genomes[..ELITE_COUNT].iter().cloned());

    let crossover_end = ELITE_COUNT + CROSSOVER_PARENT_COUNT;
    for parents in ranked_genomes[ELITE_COUNT..crossover_end].chunks_exact(2) {
        let [a, b] = parents else {
            unreachable!("crossover parents are processed in pairs");
        };

        next_generation.push(cross_genomes(a, b));
        next_generation.push(cross_genomes(b, a));
    }

    next_generation.extend((0..RANDOM_COUNT).map(|_| random_genome()));
    next_generation
}

pub fn rerun_simulation(physics: &mut Physics, mut simulation: Simulation) -> Simulation {
    let ranked_genomes = simulation
        .racers
        .iter()
        .map(|simulation_racer| simulation_racer.racer.genome.clone())
        .collect::<Vec<Genome>>();

    let next_generation = sample_next_generation(&ranked_genomes);

    for (simulation_racer, genome) in simulation.racers.iter_mut().zip(next_generation) {
        restart_racer(
            physics,
            &mut simulation_racer.racer,
            simulation_racer.spawn_position,
        );
        simulation_racer.racer.genome = genome;
    }

    simulation
}
