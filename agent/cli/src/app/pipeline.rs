use crate::domain::report::Report;
use crate::ports::CheckPlugin;

pub struct Pipeline {
    plugins: Vec<Box<dyn CheckPlugin>>,
}

impl Pipeline {
    pub fn new(plugins: Vec<Box<dyn CheckPlugin>>) -> Self {
        Self { plugins }
    }

    pub fn run(&self) -> Report {
        let mut report = Report::empty();
        for plugin in &self.plugins {
            report.findings.extend(plugin.run());
        }
        report
    }
}
