use crate::diff::EnvDiff;

/// Output format for a diff report.
#[derive(Debug, Clone, PartialEq)]
pub enum DiffFormat {
    Plain,
    Json,
}

/// Formats an `EnvDiff` into a string representation.
pub struct DiffFormatter {
    pub format: DiffFormat,
}

impl DiffFormatter {
    pub fn new(format: DiffFormat) -> Self {
        DiffFormatter { format }
    }

    pub fn render(&self, diff: &EnvDiff) -> String {
        match self.format {
            DiffFormat::Plain => diff.summary(),
            DiffFormat::Json => self.render_json(diff),
        }
    }

    fn render_json(&self, diff: &EnvDiff) -> String {
        let mut parts = Vec::new();

        let mut added_entries: Vec<String> = diff
            .added
            .iter()
            .map(|(k, v)| format!("{{\"key\":\"{}\",\"value\":\"{}\"}}", k, v))
            .collect();
        added_entries.sort();

        let mut removed_entries: Vec<String> = diff
            .removed
            .iter()
            .map(|(k, v)| format!("{{\"key\":\"{}\",\"value\":\"{}\"}}", k, v))
            .collect();
        removed_entries.sort();

        let mut changed_entries: Vec<String> = diff
            .changed
            .iter()
            .map(|(k, (old, new))| {
                format!(
                    "{{\"key\":\"{}\",\"old\":\"{}\",\"new\":\"{}\"}}",
                    k, old, new
                )
            })
            .collect();
        changed_entries.sort();

        parts.push(format!("\"added\":[{}]", added_entries.join(",")));
        parts.push(format!("\"removed\":[{}]", removed_entries.join(",")));
        parts.push(format!("\"changed\":[{}]", changed_entries.join(",")));

        format!("{{{}}}", parts.join(","))
    }
}
