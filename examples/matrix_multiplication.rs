use candle_core::{Device, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;
    let a = Tensor::new(
        &[
            [1f32, 2., 3.],
            [4., 5., 6.],
        ],
        &device,
    )?;

    let b = Tensor::new(
        &[
            [10f32, 20.],
            [30., 40.],
            [50., 60.]
        ],
        &device
    )?;
    let result = a.matmul(&b)?;

    println!("Result: {:?}", result);
    println!("Shape: {:?}", result.dims());
    println!("Values: {:?}", result.to_vec2::<f32>()?);

    Ok(())
}
