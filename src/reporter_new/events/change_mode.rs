use crate::reporter_new::events::json_event;
use crate::reporter_new::{Event, HumanOutput, JsonOutput};
use crate::ModeIntent;
use console::style;
use std::io::Write;

pub struct ChangeMode {
    mode: ModeIntent,
}

impl ChangeMode {
    fn welcome(&self, target: &str, cmd: &str, action_intent: ModeIntent) {
        let verb = match action_intent {
            ModeIntent::Find => "Determining",
            ModeIntent::Verify => "Verifying",
            ModeIntent::List | ModeIntent::Set | ModeIntent::Show => "",
        };

        let _ = self.term.write_line(
            format!(
                "{} the Minimum Supported Rust Version (MSRV) for toolchain {}",
                verb,
                style(target).bold()
            )
            .as_str(),
        );

        let _ = self.term.write_line(
            format!(
                "Using {} command {}",
                style("check").bold(),
                style(cmd).italic(),
            )
            .as_str(),
        );

        self.progress.enable_steady_tick(250);
    }
}

impl Event<HumanOutput> for ChangeMode {
    fn write_formatted_event<W>(&self, writer: &mut W)
    where
        W: Write,
    {
        if let ModeIntent::List | ModeIntent::Show = self.mode {
            return;
        }

        // self.welcome(self.toolchain, self.cmd, action);

        let _ = writer.write_all("Fetching index".as_bytes());
    }
}

impl Event<JsonOutput> for ChangeMode {
    fn write_formatted_event<W>(&self, writer: &mut W)
    where
        W: Write,
    {
        let event = json_event("fetching-index", json::object! {});
        let _ = writer.write_all(event.to_string().as_bytes());
    }
}

fn welcome_message() -> String {}
