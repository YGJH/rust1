use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

struct Network {

    neuron_counts: Vec<usize>,
    // weight[layer][neuron_idx][input_idx]
    weights: Vec<Vec<Vec<f32>>>,
    biases: Vec<Vec<f32>>,

    learning_rate: f32,
}
impl Network {
    pub fn new(
        in_dim: usize,
        hidden_layer_count: usize,
        hidden_dim: usize,
        output_dim: usize,
        learning_rate: f32,
    ) -> Self {
        let mut rng = StdRng::seed_from_u64(42); // 可重現的序列
        

        let mut neuron_counts = Vec::with_capacity(hidden_layer_count + 2);
        neuron_counts.push(in_dim);
        for _ in 0..hidden_layer_count {
            neuron_counts.push(hidden_dim);
        }
        neuron_counts.push(output_dim);
        
        
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        
        for layer in 0..neuron_counts.len()-1 {
            let in_d = neuron_counts[layer];
            let out_d = neuron_counts[layer+1];
            let weight = (0..in_d).map(|_| (0..out_d).map(|_| rng.random_range(-1.0..1.0)).collect()).collect();
            let bias = (0..out_d).map(|_| rng.random_range(-1.0..1.0)).collect();
            weights.push(weight);
            biases.push(bias);
        }
        Self {
            neuron_counts,
            weights,
            biases,
            learning_rate,
        }
    }
    // pub fn print_nodes(&self) {

    //     println!("hidden layer: ");
    //     for (i, (weights, biases)) in self.weight.iter().zip(self.bias.iter()).enumerate(){
    //         for (j , (weight , bias)) in weights.iter().zip(biases.iter()).enumerate() {
    //             println!("hidden[{}][{}]: {} {}", i, j, bias, weight);

    //         }
    //     }

    //     println!("learning rate: {}", self.learning_rate);
    // }
    fn eval(x: &[f32], w: &[f32], b: f32) -> f32 {
        x.iter().zip(w.iter()).map(|(xi, wi)| xi * wi).sum::<f32>() + b
    }

    fn loss_derivative(prediction:f32 , target:f32) -> f32 {
        2.0 * (prediction - target)
    }
    fn sigmoid(val:f32) -> f32 {
        // sigmoid
        1.0 / (1.0 + (-val).exp())
    }
    fn sigmoid_derivative(val:f32) -> f32 {
        let ds = Self::sigmoid(val);
        ds *  (1.0 - ds)
    }
    fn loss(predict_val:f32 , true_val:f32) -> f32 {
        // mse
        (true_val - predict_val) * (true_val - predict_val) 
    }
    fn forward(&mut self, input: Vec<f32>) -> (Vec<Vec<f32>> , Vec<Vec<f32>>) {
        let mut layer_outputs = Vec::new();
        let mut weighted_inputs = Vec::new();

        layer_outputs.push(input);

        for (layer, (w_row , b_layer)) in self.weights.iter().zip(self.biases.iter()).enumerate() {
            let prev_output = &layer_outputs[layer];
            eprintln!("Layer: {}" , layer);
            eprintln!("prev_outptu: {:.?}" , prev_output);

            let mut z_vec = vec![0.0 ; b_layer.len()];
            let mut a_vec = vec![0.0 ; b_layer.len()];
            for neuron in 0..b_layer.len() {
                let sum: f32 = w_row[neuron]
                    .iter()
                    .zip(prev_output.iter())
                    .map(|(&w_ij , &o_j)| w_ij * o_j)
                    .sum();
                z_vec[neuron] = sum + b_layer[neuron];
                a_vec[neuron] = Self::sigmoid( z_vec[neuron] );

            }
            weighted_inputs.push(z_vec);
            layer_outputs.push(a_vec);

        }

        return (weighted_inputs, layer_outputs);
    }
    // fn back_propagation(&mut self, mut z:Vec<Vec<f32>>,mut a:Vec<Vec<f32>>, target:f32) {
    //     let mut last_act_a =  a[a.len()-1].clone();
    //     let mut last_act_z =  z[z.len()-1].clone();
    //     let tmp_a = Vec::new();
    //     let tmp_b = Vec::new();
    //     for (i , layer) in (0..self.hidden_layers).rev().enumerate() {
    //         for (j , (_z, _a)) in last_act_z.iter().zip(last_act_a.iter()).enumerate() {
    //             let a_l = Self::loss_derivative(target , *_a);
    //             let z_l = a_l * Self::sigmoid_derivative(*_z);
    //             self.weight[i][j] -= z_l * self.learning_rate;
    //             tmp_a.push(a_l);
    //             tmp_b.push(z_l);
    //         }
    //         last_act_a = tmp_a;
    //         last_act_z = tmp_b;
    //         tmp_a = Vec::new();
    //         tmp_b = Vec::new();
    //     }
    // }

    // fn updata_weight() {

    // }


    pub fn train(&mut self, epoch:usize , input: Vec<Vec<f32>>,target: Vec<Vec<f32>>) {
        for i in 0..epoch {
            for (input , target) in input.iter().zip(target.iter()) {
                let (z, a) = self.forward(input.clone());
                // let mut err = Self::loss(prediction , *target);
                // println!("prediction: {} actual: {} loss: {}", prediction , *target , err);


            }
        }
    }


}
fn main() {
    let n = 10;
    let input: Vec<Vec<f32>> = (0..n).map(|x| vec![(x as f32)]).collect();
    let output: Vec<Vec<f32>> = (0..n).map(|x| vec![(x*x) as f32]).collect();
    eprintln!("finish gen input");
    let mut net = Network::new(
        4,
        4,
        1,
        1,
        0.333,
    );
    // net.print_nodes();
    net.train(1, input , output)
    // net.set_input(input , output);

}

