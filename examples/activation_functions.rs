use candle_core::{Device, Tensor};
use candle_nn::ops::sigmoid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    let input = Tensor::new(&[-3f32, 5., -1., 8.], &device)?;

    // ReLU
    let relu = input.relu()?;
    println!("ReLU: {:?}", relu.to_vec1::<f32>()?);

    // Sigmoid
    let sigmoid = sigmoid(&input)?;
    println!("Sigmoid: {:?}", sigmoid.to_vec1::<f32>()?);

    // Tanh
    let tanh = input.tanh()?;
    println!("Tanh: {:?}", tanh.to_vec1::<f32>()?);

    Ok(())
}