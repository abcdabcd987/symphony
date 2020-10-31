mod tfwrapper;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_session() {
        let model_pb = "resources/mnist/mnist.pb";
        let session = tfwrapper::Session::new(model_pb).unwrap();
        assert_eq!(session.hello(), 2333);
    }

    #[test]
    fn model_not_exists() {
        let model_pb = "not-exists.pb";
        assert!(tfwrapper::Session::new(model_pb).is_err());
    }
}
