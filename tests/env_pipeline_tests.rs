#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use envlayer::env_pipeline::{EnvPipeline, PipelineStep};
    use envlayer::error::EnvLayerError;

    struct UppercaseStep;

    impl PipelineStep for UppercaseStep {
        fn name(&self) -> &str {
            "uppercase"
        }

        fn process(
            &self,
            env: HashMap<String, String>,
        ) -> Result<HashMap<String, String>, EnvLayerError> {
            Ok(env.into_iter().map(|(k, v)| (k, v.to_uppercase())).collect())
        }
    }

    struct PrefixStep(String);

    impl PipelineStep for PrefixStep {
        fn name(&self) -> &str {
            "prefix"
        }

        fn process(
            &self,
            env: HashMap<String, String>,
        ) -> Result<HashMap<String, String>, EnvLayerError> {
            Ok(env
                .into_iter()
                .map(|(k, v)| (k, format!("{}{}", self.0, v)))
                .collect())
        }
    }

    fn base_env() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("KEY".to_string(), "value".to_string());
        m
    }

    #[test]
    fn test_empty_pipeline_returns_input_unchanged() {
        let pipeline = EnvPipeline::new();
        let result = pipeline.run(base_env()).unwrap();
        assert_eq!(result["KEY"], "value");
    }

    #[test]
    fn test_single_step_applied() {
        let mut pipeline = EnvPipeline::new();
        pipeline.add_step(Box::new(UppercaseStep));
        let result = pipeline.run(base_env()).unwrap();
        assert_eq!(result["KEY"], "VALUE");
    }

    #[test]
    fn test_multiple_steps_applied_in_order() {
        let mut pipeline = EnvPipeline::new();
        pipeline.add_step(Box::new(UppercaseStep));
        pipeline.add_step(Box::new(PrefixStep("PRE:".to_string())));
        let result = pipeline.run(base_env()).unwrap();
        assert_eq!(result["KEY"], "PRE:VALUE");
    }

    #[test]
    fn test_step_names_reported_correctly() {
        let mut pipeline = EnvPipeline::new();
        pipeline.add_step(Box::new(UppercaseStep));
        pipeline.add_step(Box::new(PrefixStep("X".to_string())));
        let names = pipeline.step_names();
        assert_eq!(names, vec!["uppercase", "prefix"]);
    }

    #[test]
    fn test_error_propagates_from_step() {
        struct FailStep;
        impl PipelineStep for FailStep {
            fn name(&self) -> &str { "fail" }
            fn process(&self, _env: HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError> {
                Err(EnvLayerError::InvalidValue("step failed".to_string()))
            }
        }
        let mut pipeline = EnvPipeline::new();
        pipeline.add_step(Box::new(FailStep));
        assert!(pipeline.run(base_env()).is_err());
    }
}
