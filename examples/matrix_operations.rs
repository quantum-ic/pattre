use candle_core::{Device, Tensor};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    let matrix = Tensor::new(
        &[
            [1f32, 2., 3.],
            [4., 5., 6.],
        ],
        &device
    )?;

    let other = Tensor::new(
        &[
            [10f32, 20., 30.],
            [40f32, 50., 60.]
        ],
        &device
    )?;

    let sum = matrix.add(&other)?;
    let product = matrix.mul(&other)?;
    println!("Sum: {:?}", sum);
    println!("Product: {:?}", product);

    println!("Matrix: {:?}", sum);
    println!("Shape: {:?}", sum.dims());
    println!("Rank: {}", sum.rank());
    println!("Values: {:?}", sum.to_vec2::<f32>()?);
    println!("____________________________");

    println!("Matrix: {:?}", product);
    println!("Shape: {:?}", product.dims());
    println!("Rank: {}", product.rank());
    println!("Values: {:?}", product.to_vec2::<f32>()?);

    println!("____________________________");

    println!("Matrix: {:?}", matrix);
    println!("Shape: {:?}", matrix.dims());
    println!("Rank: {}", matrix.rank());
    println!("Values: {:?}", matrix.to_vec2::<f32>()?);

    Ok(())
}