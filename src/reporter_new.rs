use crate::reporter_new::events::Event;
use std::io::Write;
use std::marker::PhantomData;

pub mod events;

/// A reporter, which writes messages of type `T` to a writer `W`.
#[derive(Debug)]
pub struct Reporter<T: ReportType, W> {
    output_type: PhantomData<T>,
    writer: W,
}

impl<T: ReportType, W> Reporter<T, W> {
    pub fn new(writer: W) -> Self {
        Self {
            output_type: PhantomData,
            writer,
        }
    }
}

impl<T, W> Report<T> for Reporter<T, W>
where
    T: ReportType,
    W: std::io::Write,
{
    fn report_event<E>(&mut self, event: E)
    where
        E: Event<T>,
    {
        let writer = &mut self.writer;
        event.write_formatted_event(writer);
    }
}

/// Trait which handle reporting an event, to be implemented by a specific reporter
pub trait Report<T> {
    fn report_event<E>(&mut self, event: E)
    where
        T: ReportType,
        E: Event<T>;
}

pub trait ReportType {}

pub struct HumanOutput;
pub struct JsonOutput;
pub struct OutputDisabled;

impl ReportType for HumanOutput {}
impl ReportType for JsonOutput {}
impl ReportType for OutputDisabled {}

// Blanket impl for all disabled outputs
impl<T> Event<OutputDisabled> for T {
    fn write_formatted_event<W>(&self, _writer: &mut W)
    where
        W: std::io::Write,
    {
    }
}
