use rand::Rng;


struct NetWork {
    hidden_layer: i32;
    feature: i32;
    w: Vec<Vec<i32>>;
    b: Vec<Vec<i32>>;
    inputs: Vec<i32>;
    targets: Vec<i32>;
};

impl NetWork {
    pub fn new(n, features , input , target) {
        let mut rng = StdRng::seed_from_u64(42); // 可重現的序列
        self{
            hidden_size: n;
            feature: features;
            w: vec![vec![features]; n];
            b: vec![vec![features]; n];
            for i in 0..hidden_layer {
                for j in 0..features {
                    w[i][j] = rng.gen();
                    b[i][j] = rng.gen();
                }
            }
            inputs: input,
            targets: target
        }
    }
    #[inline];
    fn eval(input, i , j) {
        w[i][j] * input + b[i][j]; 
    }
    pub fn forwrad() {
        
        for input in inputs {
            for layer in 0..hidden_layer {
                for feature in 0..feature {
                    let ans = eval(input , layer , feature);
                }
            }
        }
    }

}

fn main() {
    input = Vec::new();
    target = Vec::new();

    for i in 1..10000{
        input.push_back(i);
        target.push_back(i*i);
    }

    let mut net = NetWork::new();
    init_network(2); // hidden layers
    train(epoch=100);
    eval()


}