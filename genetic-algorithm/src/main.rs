use genome::Genome;
use rand::Rng;

mod genome;

fn main() {
    let mut population = generate_population(100);
    for generation in 0..100 {
        let fitness_values = evaluate_fitness_of_population(&population);
        println!("Generation {}:\n", generation + 1);
        print_population(&population);
        print_fitness_values(&fitness_values);
        print_average_fitness(&fitness_values);
        let pairings = generate_pairings(&population);
        print_pairings(&pairings);
        population = next_generation(&population, pairings);
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

fn generate_pairings(population: &Vec<Genome>) -> Vec<(usize, usize)> {
    let fitness_values = evaluate_fitness_of_population(population);
    (0..population.len())
        .map(|_| {
            let genome_one_index = select_genome(&fitness_values, None);
            let genome_two_index = select_genome(&fitness_values, Some(genome_one_index));
            (genome_one_index, genome_two_index)
        })
        .collect()
}

fn next_generation(
    population: &Vec<genome::Genome>,
    pairings: Vec<(usize, usize)>,
) -> Vec<genome::Genome> {
    pairings
        .iter()
        .map(|(genome_one_index, genome_two_index)| {
            let new_genome = population[*genome_one_index].cross(&population[*genome_two_index]);
            new_genome
        })
        .collect()
}

fn evaluate_fitness_of_population(population: &Vec<genome::Genome>) -> Vec<usize> {
    let mut fitness_values = Vec::with_capacity(population.len());
    for genome in population {
        let fitness = evaluate_fitness(genome);
        fitness_values.push(fitness);
    }
    fitness_values
}

fn evaluate_fitness(Genome { p, i, d }: &genome::Genome) -> usize {
    (1000.0 * 1.0 / (p + i + d)).round() as usize
}

fn select_genome(fitness_values: &Vec<usize>, ignore: Option<usize>) -> usize {
    let mut index = selection(fitness_values);
    if let Some(ignore) = ignore {
        while index == ignore {
            index = selection(fitness_values);
        }
    }
    index
}

fn selection(fitness_values: &Vec<usize>) -> usize {
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

fn print_fitness_values(fitness_values: &Vec<usize>) {
    println!("\tPopulation Fitness:\n");
    let sum_of_fitness_values = fitness_values.iter().sum::<usize>();
    for (index, fitness) in fitness_values.iter().enumerate() {
        let percentage = (fitness * 100) / sum_of_fitness_values;
        println!("\t\t{}) Fitness: {} ({}%)", index + 1, fitness, percentage);
    }
    println!();
}

fn print_average_fitness(fitness_values: &Vec<usize>) {
    let sum_of_fitness_values = fitness_values.iter().sum::<usize>();
    let average_fitness = sum_of_fitness_values / fitness_values.len();
    println!("\tAverage Fitness: {}\n", average_fitness);
}

fn print_pairings(pairings: &Vec<(usize, usize)>) {
    println!("\tPairings:\n");
    for (index, (genome_one_index, genome_two_index)) in pairings.iter().enumerate() {
        println!(
            "\t\t{}) {} x {}",
            index + 1,
            genome_one_index,
            genome_two_index
        );
    }
    println!();
}
