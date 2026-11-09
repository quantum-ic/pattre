use candle_core::{Device, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    let predictions = Tensor::new(
        &[70f32, 80., 90.],
        &device
    )?;
    let targets = Tensor::new(
        &[100f32, 100., 100.],
        &device
    )?;

    let errors = predictions.sub(&targets)?;
    let squared_errors = errors.sqr()?;
    let loss = squared_errors.mean_all()?;


    println!("Predictions: {:?}", predictions);
    println!("Targets: {:?}", targets);
    println!("Squared Errors: {:?}", squared_errors);
    println!("MSE Loss: {:?}", loss);


Ok(())
}