use candle_core::{Device, Var};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    let w = Var::new(3f32, &device)?;

    let loss = w.as_tensor().sqr()?;

    let grads = loss.backward()?;

    let gradient = grads.get(&w).unwrap();

    println!("Weight: {:?}", w.as_tensor().to_scalar::<f32>()?);
    println!("Loss: {:?}", loss.to_scalar::<f32>()?);
    println!("Gradient: {:?}", gradient.to_scalar::<f32>()?);
    
    Ok(())
}
