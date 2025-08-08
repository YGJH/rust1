use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// 一個具備任意隱藏層結構的多層感知器 (MLP)
struct Network {
    /// 每層的神經元數量：
    ///   neuron_counts[0] = 輸入層維度
    ///   neuron_counts[last] = 輸出層維度
    neuron_counts: Vec<usize>,
    /// 每層之間的權重矩陣：weights[layer][neuron_index][prev_neuron_index]
    weights: Vec<Vec<Vec<f32>>> ,
    /// 每層的偏差向量：biases[layer][neuron_index]
    biases: Vec<Vec<f32>>,
    /// 學習率
    learning_rate: f32,
}

impl Network {
    /// 建構函數：指定輸入維度、隱藏層數、每個隱藏層的神經元數、輸出維度，以及學習率
    pub fn new(
        input_dim: usize,
        hidden_layer_count: usize,
        hidden_dim: usize,
        output_dim: usize,
        learning_rate: f32,
    ) -> Self {
        let mut rng = StdRng::seed_from_u64(42);

        // 組成 neuron_counts：輸入層 → 隱藏層 × hidden_layer_count → 輸出層
        let mut neuron_counts = Vec::with_capacity(hidden_layer_count + 2);
        neuron_counts.push(input_dim);
        for _ in 0..hidden_layer_count {
            neuron_counts.push(hidden_dim);
        }
        neuron_counts.push(output_dim);

        // 初始化 weights、biases
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        // 每個 layer 只存 layer → next_layer 的參數
        for layer in 0..neuron_counts.len() - 1 {
            let in_size = neuron_counts[layer];
            let out_size = neuron_counts[layer + 1];
            // weight 矩陣: out_size × in_size
            let w: Vec<Vec<f32>> = (0..out_size)
                .map(|_| (0..in_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
                .collect();
            let b: Vec<f32> = (0..out_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
            weights.push(w);
            biases.push(b);
        }

        Network { neuron_counts, weights, biases, learning_rate }
    }

    /// Sigmoid 函數
    fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }
    /// Sigmoid 導數，輸入已為 sigmoid(x) 的值
    fn sigmoid_derivative(sig_x: f32) -> f32 {
        sig_x * (1.0 - sig_x)
    }

    /// 前向傳播，回傳：
    /// - weighted_inputs: 每層的 z 向量 (線性組合結果)
    /// - layer_outputs: 每層的激活輸出 (包含輸入層)
    pub fn forward(&self, input_vector: &[f32]) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        let mut layer_outputs = Vec::new();
        let mut weighted_inputs = Vec::new();

        // 第一層輸出就是輸入向量本身
        layer_outputs.push(input_vector.to_vec());

        // 依序計算各層
        for (layer_idx, (w_layer, b_layer)) in self.weights.iter().zip(self.biases.iter()).enumerate() {
            let prev_output = &layer_outputs[layer_idx];
            let mut z_vec = vec![0.0; b_layer.len()];
            let mut a_vec = vec![0.0; b_layer.len()];

            for neuron in 0..b_layer.len() {
                // 計算加權總和 z = w·prev_output + b
                let weighted_sum: f32 = w_layer[neuron]
                    .iter()
                    .zip(prev_output.iter())
                    .map(|(&w_ij, &o_j)| w_ij * o_j)
                    .sum();
                z_vec[neuron] = weighted_sum + b_layer[neuron];
                a_vec[neuron] = Self::sigmoid(z_vec[neuron]);
            }

            weighted_inputs.push(z_vec);
            layer_outputs.push(a_vec);
        }

        (weighted_inputs, layer_outputs)
    }

    /// 單筆訓練 (forward + backward)，傳回 MSE loss
    pub fn train_step(&mut self, input_vector: &[f32], target_vector: &[f32]) -> f32 {
        // Forward
        let (z_vectors, outputs) = self.forward(input_vector);
        let layer_count = self.weights.len();

        // 準備存放各層的誤差 delta
        let mut deltas: Vec<Vec<f32>> = Vec::with_capacity(layer_count);

        // 1) 計算輸出層誤差
        let final_output = &outputs[layer_count + 1 - 1]; // outputs 最後一層
        let mut delta_output = vec![0.0; final_output.len()];
        for i in 0..final_output.len() {
            let error = final_output[i] - target_vector[i];
            delta_output[i] = error * Self::sigmoid_derivative(final_output[i]);
        }
        deltas.push(delta_output);

        // 2) 反向計算隱藏層誤差
        for layer in (0..layer_count - 1).rev() {
            let next_weights = &self.weights[layer + 1];
            let next_delta = &deltas[0];
            let activation = &outputs[layer + 1];
            let mut delta_hidden = vec![0.0; activation.len()];

            for i in 0..activation.len() {
                let propagated_error: f32 = next_weights
                    .iter()
                    .zip(next_delta.iter())
                    .map(|(w_row, &d_j)| w_row[i] * d_j)
                    .sum();
                delta_hidden[i] = propagated_error * Self::sigmoid_derivative(activation[i]);
            }
            deltas.insert(0, delta_hidden);
        }

        // 3) 更新權重與偏差
        for layer in 0..layer_count {
            let input_activation = &outputs[layer];
            let layer_delta = &deltas[layer];

            for neuron in 0..layer_delta.len() {
                for w_index in 0..input_activation.len() {
                    self.weights[layer][neuron][w_index] -=
                        self.learning_rate * layer_delta[neuron] * input_activation[w_index];
                }
                self.biases[layer][neuron] -= self.learning_rate * layer_delta[neuron];
            }
        }

        // 4) 計算並回傳 MSE loss
        let prediction = &outputs[layer_count];
        let mse_loss: f32 = prediction
            .iter()
            .zip(target_vector.iter())
            .map(|(&pred, &truth)| (truth - pred).powi(2))
            .sum::<f32>()
            / (prediction.len() as f32);

        mse_loss
    }

    /// 多筆訓練：重複 epochs 次
    pub fn train(&mut self, dataset_inputs: &[Vec<f32>], dataset_targets: &[Vec<f32>], epochs: usize) {
        for epoch in 0..epochs {
            let mut total_loss = 0.0;
            for (inp, tgt) in dataset_inputs.iter().zip(dataset_targets.iter()) {
                total_loss += self.train_step(inp, tgt);
            }
            if epoch % 1000 == 0 {
                println!("Epoch {:>5}: 平均損失 = {:.6}", epoch, total_loss / (dataset_inputs.len() as f32));
            }
        }
    }
}

fn main() {
    // 範例：x -> x^2 (已縮放至 [0,1])
    let sample_size = 1000;
    let mut inputs = Vec::with_capacity(sample_size);
    let mut labels = Vec::with_capacity(sample_size);
    for i in 0..sample_size {
        let x = i as f32 / sample_size as f32;
        inputs.push(vec![x]);
        labels.push(vec![x * x]);
    }

    // 建立網路：輸入 1 維 → 兩層隱藏層(各16) → 輸出 1 維，學習率 0.5
    let mut network = Network::new(1, 2, 16, 1, 0.5);
    network.train(&inputs, &labels, 10_000);

    // 測試: x=0.8
    let test_input = vec![0.8];
    let (_, outputs) = network.forward(&test_input);
    println!("輸入 0.8，預測輸出 = {:.4}", outputs.last().unwrap()[0]);
}
