use candle_core::{Device, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    let sequence = Tensor::new(
        &[1f32, 3., 5., 7., 9.],
        &device,
    )?;

    println!("Tensor: {:?}", sequence);
    println!("Shape: {:?}", sequence.dims());
    println!("Rank: {:?}", sequence.rank());
    println!("Data type: {:?}", sequence.dtype());
    println!("Values: {:?}", sequence.to_vec1::<f32>()?);



    Ok(())
}