use jedvek::Matrix2D;

fn main() {
    let initial_a_signal = Matrix2D::from(&[[1, 0, 1, 0, 1, 0, 0, 1]]);

    // Initial B activations
    let b1_1 = [1, 1, 1, 0, 1, 0, 0, 1];
    let b2_1 = [1; 8];
    let b3_1 = [1, 0, 1, 0, 1, 1, 0, 1];
    let b4_1 = [0, 1, 1, 1, 0, 1, 1, 0];
    let b5_1 = [1, 0, 1, 1, 1, 0, 0, 1];
    let b6_1 = [1, 1, 0, 1, 1, 0, 1, 0];
    let b7_1 = [1, 0, 1, 0, 1, 0, 1, 1];
    let b8_1 = [1, 0, 1, 0, 1, 1, 0, 1];

    let b_neuron_initial = Matrix2D::from(&[b1_1, b2_1, b3_1, b4_1, b5_1, b6_1, b7_1, b8_1]);
    let signal_threshold = 3;
    let mut x_ms = send_signal(&initial_a_signal, &b_neuron_initial, signal_threshold);
    println!(" 0 ms: {}\n10 ms: {x_ms}", Matrix2D::from(&[[0; 8]]));

    // Secondary B activations
    let b1_2 = [1, 0, 1, 1, 0, 1, 1, 1];
    let b2_2 = [1, 1, 0, 1, 1, 1, 0, 0];
    let b3_2 = [0, 1, 1, 1, 0, 1, 1, 0];
    let b4_2 = [1, 0, 1, 1, 1, 1, 0, 1];
    let b5_2 = [0, 1, 0, 1, 0, 1, 1, 0];
    let b6_2 = [1, 0, 1, 1, 0, 1, 1, 1];
    let b7_2 = [0, 0, 1, 1, 1, 1, 0, 1];
    let b8_2 = [1, 1, 0, 1, 0, 1, 1, 1];
    let b_neuron_secondary = Matrix2D::from(&[b1_2, b2_2, b3_2, b4_2, b5_2, b6_2, b7_2, b8_2]);

    // Calculate 20ms to 40ms in 10ms increments
    for i in 2..=4 {
        x_ms = send_signal(&x_ms, &b_neuron_secondary, signal_threshold);
        println!("{} ms: {x_ms}", i * 10)
    }
}

fn send_signal(input_singal: &Matrix2D<u8>, synapse: &Matrix2D<u8>, threshold: u8) -> Matrix2D<u8> {
    let length = input_singal.width;
    let mut output_signal = Matrix2D::full(0, input_singal.height, length);
    for (i, neuron) in synapse.into_iter().enumerate() {
        for j in 0..length {
            // signal and activator are both 1
            if input_singal[0][j] + neuron[j] == 2 {
                output_signal[0][i] += 1;
            }
        }
    }
    for signal in &mut output_signal[0] {
        // If signal of one neuron meets a threshold, neuron is "active"
        if *signal >= threshold {
            *signal = 1;
        } else {
            *signal = 0;
        }
    }
    output_signal
}

