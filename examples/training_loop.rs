use candle_core::{DType, Device, Tensor};
use candle_nn::{Linear, Module, Optimizer, SGD, VarBuilder, VarMap, linear};

struct SimpleModel {
    layer: Linear,
}
impl SimpleModel {
    fn new(var_builder: VarBuilder<'_>) -> candle_core::Result<Self> {
        let layer = linear(1, 1, var_builder.pp("layer"))?;

        Ok(Self { layer })
    }
    fn forward(&self, input: &Tensor) -> candle_core::Result<Tensor> {
        self.layer.forward(input)
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;
    let varmap = VarMap::new();

    let var_builder = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = SimpleModel::new(var_builder)?;

    let mut optimizer = SGD::new(varmap.all_vars(), 0.01)?;
    /*for (index, var) in varmap.all_vars().iter().enumerate() {
        println!("Variable {index}: {:?}", var.as_tensor().to_vec2::<f32>());
    }*/

    let input = Tensor::new(&[[1f32], [2.], [3.]], &device)?;

    let targets = Tensor::new(&[[5f32], [10.], [15.]], &device)?;

    for epoch in 0..=1000 {
        let predictions = model.forward(&input)?;
        let loss = predictions.sub(&targets)?.sqr()?.mean_all()?;
        let grads = loss.backward()?;
        optimizer.step(&grads)?;

        if epoch % 10 == 0 {
            println!("Epoch: {epoch}, Loss: {}", loss.to_scalar::<f32>()?);
        }
    }

    let predictions = model.forward(&input)?;

    let loss = predictions.sub(&targets)?.sqr()?.mean_all()?;
    println!("Predictions: {:?}", predictions);
    println!("Targets: {:?}", targets);
    println!("Loss: {:?}", loss.to_scalar::<f32>()?);

    let new_input = Tensor::new(&[[10f32]], &device)?;

    let prediction = model.forward(&new_input)?;

    println!("Prediction: {:?}", prediction.to_vec2::<f32>()?);

    Ok(())
}
