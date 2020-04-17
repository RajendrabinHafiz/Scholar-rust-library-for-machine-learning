use scholar::dataset::Dataset;
use scholar::net::{NeuralNet, SIGMOID};

fn main() -> anyhow::Result<()> {
    let dataset = Dataset::from_csv("examples/iris.csv", false, 4)?;
    let (training_data, testing_data) = dataset.split(0.75);

    let mut brain = NeuralNet::new(&[4, 10, 10, 1], SIGMOID);
    brain.train(training_data, 25_000, 0.01);

    println!("Accuracy: {:.2}%", (1.0 - brain.test(testing_data)) * 100.0);

    Ok(())
}