mod tfwrapper;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn mnist_config() -> tfwrapper::SessionConfig {
        tfwrapper::SessionConfig {
            model_pb: "resources/mnist/mnist.pb".to_owned(),
            max_batch: 10,
            input_shape: vec![28, 28],
            input_name: "flatten_input".to_owned(),
            output_name: "output/Softmax".to_owned(),
        }
    }

    fn mnist_image(path: &str) -> Vec<f32> {
        fs::read_to_string(path)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.trim().parse::<f32>().unwrap() / 255.0)
            .collect()
    }

    fn mnist_dataset() -> Vec<(Vec<f32>, usize)> {
        vec![
            (mnist_image("resources/mnist/xtest_208.txt"), 2),
            (mnist_image("resources/mnist/xtest_233.txt"), 8),
            (mnist_image("resources/mnist/xtest_666.txt"), 7),
            (mnist_image("resources/mnist/xtest_1115.txt"), 5),
            (mnist_image("resources/mnist/xtest_1234.txt"), 8),
        ]
    }

    fn argmax<T: PartialOrd>(xs: &[T]) -> Option<usize> {
        if xs.is_empty() {
            return None;
        }
        let mut arg = 0usize;
        for (i, v) in xs.iter().enumerate() {
            if v > &xs[arg] {
                arg = i;
            }
        }
        Some(arg)
    }

    #[test]
    fn new_session() {
        let config = mnist_config();
        let session = tfwrapper::Session::new(config).unwrap();
        assert_eq!(session.hello(), 2333);
    }

    #[test]
    fn model_not_exists() {
        let mut config = mnist_config();
        config.model_pb = "not-exists.pb".to_owned();
        assert!(tfwrapper::Session::new(config).is_err());
    }

    #[test]
    fn forward() {
        let dataset = mnist_dataset();
        let config = mnist_config();
        let mut session = tfwrapper::Session::new(config).unwrap();
        let inputs = session.input_tensor();
        for (index, (image, _)) in dataset.iter().enumerate() {
            inputs.at(index).copy_from(image).unwrap();
        }
        let outputs = session.forward(dataset.len()).unwrap();
        for (index, (_, label)) in dataset.iter().enumerate() {
            let prob = outputs.at(index).read().unwrap();
            assert_eq!(argmax(&prob), Some(*label));
        }
    }
}
