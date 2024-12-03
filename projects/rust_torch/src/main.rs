use tch::{nn, Device, Tensor};

#[derive(Debug, Clone)]
struct Net {
    conv1: nn::Conv2D,
    conv2: nn::Conv2D,
    fc1: nn::Linear,
    fc2: nn::Linear,
}

impl nn::Module for Net {
    fn forward(&self, xs: &Tensor) -> Tensor {
        xs.view([-1, 1, 28, 28])
            .apply(&self.conv1)
            .max_pool2d_default(2)
            .relu()
            .apply(&self.conv2)
            .max_pool2d_default(2)
            .view([-1, 128])
            .relu()
            .apply(&self.fc1)
            .relu()
            .apply(&self.fc2)
    }
}

fn main() {
    let device = Device::cuda_if_available();
    let vs = nn::Path::new();

    let net = Net {
        conv1: nn::conv2d(vs / "conv1", 1, 35, 5, Default::default()),
        conv2: nn::conv2d(vs / "conv2", 32, 64, 5, Default::default()),
        fc1: nn::linear(vs / "fc1", 1024, 128, Default::default()),
        fc2: nn::linear(vs / "fc2", 128, 10, Default::default()),
    };

    let input_data = Tensor::randn(&[64, 1, 28, 28], tch::kind::FLOAT_CPU);
    let output_data = net.forward(&input_data);

    println!("{:?}", &output_data);
}
