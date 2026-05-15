#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use envlayer::pipeline_builder::PipelineBuilder;
    use envlayer::error::EnvLayerError;

    fn base_env() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("APP_ENV".to_string(), "development".to_string());
        m.insert("LOG_LEVEL".to_string(), "debug".to_string());
        m
    }

    #[test]
    fn test_builder_with_no_steps() {
        let pipeline = PipelineBuilder::new().build();
        let result = pipeline.run(base_env()).unwrap();
        assert_eq!(result["APP_ENV"], "development");
    }

    #[test]
    fn test_builder_single_fn_step() {
        let pipeline = PipelineBuilder::new()
            .step("trim", |env| {
                Ok(env.into_iter().map(|(k, v)| (k, v.trim().to_string())).collect())
            })
            .build();
        let mut input = HashMap::new();
        input.insert("KEY".to_string(), "  hello  ".to_string());
        let result = pipeline.run(input).unwrap();
        assert_eq!(result["KEY"], "hello");
    }

    #[test]
    fn test_builder_chained_steps() {
        let pipeline = PipelineBuilder::new()
            .step("uppercase_keys", |env| {
                Ok(env.into_iter().map(|(k, v)| (k.to_uppercase(), v)).collect())
            })
            .step("add_default", |mut env| {
                env.entry("TIMEOUT".to_string()).or_insert_with(|| "30".to_string());
                Ok(env)
            })
            .build();
        let result = pipeline.run(base_env()).unwrap();
        assert!(result.contains_key("APP_ENV"));
        assert_eq!(result["TIMEOUT"], "30");
    }

    #[test]
    fn test_builder_step_error_halts_pipeline() {
        let pipeline = PipelineBuilder::new()
            .step("ok_step", |env| Ok(env))
            .step("bad_step", |_env| {
                Err(EnvLayerError::InvalidValue("bad".to_string()))
            })
            .step("never_reached", |env| Ok(env))
            .build();
        assert!(pipeline.run(base_env()).is_err());
    }

    #[test]
    fn test_builder_filter_step() {
        let pipeline = PipelineBuilder::new()
            .step("filter_log", |env| {
                Ok(env.into_iter().filter(|(k, _)| !k.starts_with("LOG")).collect())
            })
            .build();
        let result = pipeline.run(base_env()).unwrap();
        assert!(!result.contains_key("LOG_LEVEL"));
        assert!(result.contains_key("APP_ENV"));
    }
}
