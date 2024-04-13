use unda::core::network::Network;
use unda::core::layer::{methods::activations::Activations,
layers::{LayerTypes::DENSE, InputTypes}};

fn main() {
    let mut heart_model = Network::new(4);

    let input_size: u8 = 3;

    heart_model.set_input(InputTypes::DENSE(input_size as usize));

    heart_model.add_layer(DENSE(1, Activations::SIGMOID, 0.01));

    heart_model.compile();
}
