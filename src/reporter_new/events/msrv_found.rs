// -- Example event, which implements an Event

use crate::manifest::bare_version::BareVersion;
use crate::reporter_new::{Event, HumanOutput, JsonOutput};

struct MsrvFoundEvent {
    msrv: BareVersion,
}

impl MsrvFoundEvent {}

impl Event<HumanOutput> for MsrvFoundEvent {
    fn write_formatted_event<W>(&self, writer: &mut W)
    where
        W: std::io::Write,
    {
        let _ = writer.write_fmt(format_args!("Message: {}", &self.msrv));
    }
}

impl Event<JsonOutput> for MsrvFoundEvent {
    fn write_formatted_event<W>(&self, writer: &mut W)
    where
        W: std::io::Write,
    {
        let _ = writer.write_fmt(format_args!("{{ \"message\": \"{}\" }}", &self.msrv));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::bare_version::BareVersion;
    use crate::reporter_new::{JsonOutput, Report, Reporter};

    #[test]
    fn test_case() {
        let buffer = Vec::<u8>::new();
        let mut reporter: Reporter<JsonOutput, Vec<u8>> = Reporter::new(buffer);

        let event = MsrvFoundEvent {
            msrv: BareVersion::TwoComponents(1, 2),
        };

        reporter.report_event(event);
    }
}
