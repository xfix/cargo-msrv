use crate::manifest::bare_version::BareVersion;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug)]
pub struct Reporter<T: OutputType, W> {
    output_type: T,
    writer: W,
}

impl<W: std::io::Write> Report<HumanOutput> for Reporter<HumanOutput, W> {
    fn report_event<E>(&mut self, event: E)
    where
        E: Event<HumanOutput>,
    {
        let writer = &mut self.writer;
        event.write_event(writer);
    }
}

impl<W: std::io::Write> Report<JsonOutput> for Reporter<JsonOutput, W> {
    fn report_event<E>(&mut self, event: E)
    where
        E: Event<JsonOutput>,
    {
        let writer = &mut self.writer;
        event.write_event(writer);
    }
}

// -- Traits which handle reporting an event, to be implemented by a specific reporter

pub trait Report<T> {
    fn report_event<E>(&mut self, event: E)
    where
        T: OutputType,
        E: Event<T>;
}

pub trait OutputType {}

pub struct HumanOutput;
pub struct JsonOutput;

impl OutputType for HumanOutput {}
impl OutputType for JsonOutput {}

pub trait Event<T> {
    fn write_event<W>(&self, writer: &mut W)
    where
        W: std::io::Write;
}

// -- Example event, which implements an Event

struct ExampleEvent {
    msrv: BareVersion,
}

impl Event<HumanOutput> for ExampleEvent {
    fn write_event<W>(&self, writer: &mut W)
    where
        W: std::io::Write,
    {
        let _ = writer.write_fmt(format_args!("Message: {}", "Hello Human!"));
    }
}

impl Event<JsonOutput> for ExampleEvent {
    fn write_event<W>(&self, writer: &mut W)
    where
        W: std::io::Write,
    {
        let _ = writer.write_fmt(format_args!("{{ \"message\": \"{}\" }}", "Hello Json!"));
    }
}

#[cfg(test)]
mod tests {
    use crate::manifest::bare_version::BareVersion;
    use crate::reporter_new::{ExampleEvent, JsonOutput, Report, Reporter};

    #[test]
    fn test_case() {
        let mut reporter = Reporter {
            output_type: JsonOutput,
            writer: std::io::stdout(),
        };

        let event = ExampleEvent {
            msrv: BareVersion::TwoComponents(1, 2),
        };

        reporter.report_event(event);
    }
}
