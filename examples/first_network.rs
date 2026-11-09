use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder, VarMap, linear};

struct SimpleNet {
    layer1: candle_nn::Linear,
    layer2: candle_nn::Linear,
}
impl SimpleNet {
    fn new(var_builder: VarBuilder<'_>) -> candle_core::Result<Self> {
        let layer1 = linear(3, 4, var_builder.pp("layer1"))?;
        let layer2 = linear(4, 2, var_builder.pp("layer2"))?;
        Ok(Self { layer1, layer2 })
    }
    fn forward(&self, input: &Tensor) -> candle_core::Result<Tensor> {
        let hidden = self.layer1.forward(input)?;
        let hidden = hidden.relu()?;

        let output = self.layer2.forward(&hidden)?;
        Ok(output)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    let varmap = VarMap::new();
    let var_builder = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = SimpleNet::new(var_builder)?;

    let input = Tensor::new(
        &[
            [1f32, 2., 3.],
            [4., 5., 6.]
        ],
        &device
    )?;

    let output = model.forward(&input)?;

    println!("Input : {:?}", input.dims());
    println!("Output Shape : {:?}", output.dims());
    println!("Output : {:?}", output);

    Ok(())
}
