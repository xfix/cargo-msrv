use crate::reporter_new::events::json_event;
use crate::reporter_new::{Event, HumanOutput, JsonOutput};
use std::io::Write;

pub struct FetchingIndex;

impl Event<HumanOutput> for FetchingIndex {
    fn write_formatted_event<W>(&self, writer: &mut W)
    where
        W: Write,
    {
        let _ = writer.write("Fetching index".as_bytes());
    }
}

impl Event<JsonOutput> for FetchingIndex {
    fn write_formatted_event<W>(&self, writer: &mut W)
    where
        W: Write,
    {
        let event = json_event("fetching-index", json::object! {});
        let _ = writer.write_all(event.to_string().as_bytes());
    }
}
