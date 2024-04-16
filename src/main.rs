use std::error::Error;

use heart_attack_dnn::heart_data::heart_model::HeartModel;
use unda::core::data::input::Input;
use unda::core::layer::methods::errors::ErrorTypes;
use unda::core::network::Network;
use unda::core::layer::{methods::activations::Activations,
layers::{LayerTypes::DENSE, InputTypes}};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let mut heart_model = Network::new(301);

        let (inputs, outputs) = HeartModel::from_file("src/heart_data/data/heart.csv")?;
        let mut inputs_dyn: Vec<&dyn Input> = vec![];

        inputs.iter().for_each(|inp| {
            inputs_dyn.push(inp);
        });

        heart_model.set_input(InputTypes::DENSE(inputs[0].len()));

        heart_model.add_layer(DENSE(8, Activations::TANH, 0.0001));
        heart_model.add_layer(DENSE(32, Activations::TANH, 0.0001));
        heart_model.add_layer(DENSE(16, Activations::TANH, 0.0001));

        heart_model.add_layer(DENSE(1, Activations::SIGMOID, 0.0001));

        heart_model.compile();

        heart_model.fit(&inputs_dyn, &outputs, 5, ErrorTypes::CategoricalCrossEntropy);

        heart_model.serialize_unda_fmt(&format!("heart_attack_tanh_{:.2}.unda", heart_model.loss * 100f32));
    }

    Ok(())
}
