use burn::{
    tensor::Tensor,
    backend::Candle
};

type Backend = Candle;

fn main() {

    // Create tensors
    let a: Tensor<Backend, 1> = Tensor::from_floats([1.0, 2.0, 3.0]);
    let b: Tensor<Backend, 1> = Tensor::from_floats([4.0, 5.0, 6.0]);


    // Add element-wise
    let c = a.add(b);

    println!("{:?}", c);
}