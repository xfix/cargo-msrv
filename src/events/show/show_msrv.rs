use crate::manifest::bare_version::BareVersion;
use crate::reporter_new::{Event, HumanOutput, JsonOutput};
use json::object::Object;
use json::JsonValue;
use std::io::Write;

pub struct ShowMsrvEvent<'e> {
    msrv: Option<BareVersion>,
    toolchain_spec: &'e str,
}

impl<'e> Event<JsonOutput> for ShowMsrvEvent<'e> {
    fn write_event<W>(&self, writer: &mut W)
    where
        W: Write,
    {
        let mut object = Object::new();
        object.insert("reason", "show-complete".into());
        object.insert("mode", "show-msrv".into());
        object.insert("toolchain", self.toolchain_spec.into());

        if let Some(ref msrv) = self.msrv {
            object.insert("msrv", format!("{}", msrv).into());
        }

        let _ = writer.write_fmt(format_args!("{}", JsonValue::Object(object)));
    }
}
