use crate::domain::report::{Finding, Report};

pub trait CheckPlugin {
    fn name(&self) -> &'static str;
    fn run(&self) -> Vec<Finding>;
}

pub trait ReportSink {
    fn submit(&self, report: &Report) -> Result<(), String>;
}
