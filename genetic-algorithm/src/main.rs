use genome::Genome;
use rand::{rngs, Rng};

mod genome;

fn main() {
    let mut population = generate_population(10);
    for generation in 0..100 {
        let fitness_values = evaluate_fitness_of_population(&population);
        print!("Generation {}: ", generation + 1);
        println!(
            "{}",
            fitness_values.iter().sum::<i32>() / population.len() as i32
        );
        // print_population(&population);
        // println!("\tFitness:");
        // print_fitness_values(&fitness_values);
        // println!("\tPairings:");
        let mut new_population = Vec::with_capacity(population.len());
        for index in 0..population.len() {
            let genome_one_index = select_genome(&fitness_values);
            let genome_two_index = select_genome(&fitness_values);
            let new_genome = population[genome_one_index].cross(&population[genome_two_index]);
            // println!(
            //     "\t\t{}) {} x {} = {:?}",
            //     index + 1,
            //     genome_one_index,
            //     genome_two_index,
            //     new_genome
            // );
            new_population.push(new_genome);
        }
        population = new_population;
    }
    print_population(&population)
}

fn generate_population(size: usize) -> Vec<genome::Genome> {
    let mut population = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        let p = rng.gen::<f32>() * 10.0;
        let i = rng.gen::<f32>() * 10.0;
        let d = rng.gen::<f32>() * 10.0;
        population.push(genome::Genome::new(p, i, d));
    }
    population
}

fn evaluate_fitness_of_population(population: &Vec<genome::Genome>) -> Vec<i32> {
    let mut fitness_values = Vec::with_capacity(population.len());
    for genome in population {
        let fitness = evaluate_fitness(genome);
        fitness_values.push(fitness);
    }
    fitness_values
}

fn evaluate_fitness(Genome { p, i, d }: &genome::Genome) -> i32 {
    (1000.0 * 1.0 / (p + i + d)).round() as i32
}

fn select_genome(fitness_values: &Vec<i32>) -> usize {
    let mut rng = rand::thread_rng();
    let sum_of_fitness_values = fitness_values.iter().sum();
    let roll = rng.gen_range(1..=sum_of_fitness_values);
    fitness_values
        .iter()
        .enumerate()
        .fold(
            (0, sum_of_fitness_values),
            |(mut index, upper_boundary), (i, fitness)| {
                if roll <= upper_boundary && roll > upper_boundary - fitness {
                    index = i;
                }
                (index, upper_boundary - fitness)
            },
        )
        .0
}

fn print_population(population: &Vec<genome::Genome>) {
    for (index, genome) in population.iter().enumerate() {
        println!("\t{}) {:?}", index + 1, genome);
    }
    println!();
}

fn print_fitness_values(fitness_values: &Vec<i32>) {
    for (index, fitness) in fitness_values.iter().enumerate() {
        println!("\t\t{}) Fitness: {}", index + 1, fitness);
    }
    println!();
}
