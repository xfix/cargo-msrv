mod change_mode;
mod fetching_index;
mod msrv_found;

pub use fetching_index::FetchingIndex;

fn json_event(message_type: &str, value: json::JsonValue) -> json::JsonValue {
    json::object! {
        // The message type
        reason: message_type,
        // Meta data describing cargo-msrv
        meta: {
            // The cargo-msrv version, currently also doubling as "json format version"
            version: env!("CARGO_PKG_VERSION"),
        },
        // The message value
        value: value,
    }
}

pub trait Event<T> {
    fn write_formatted_event<W>(&self, writer: &mut W)
    where
        W: std::io::Write;
}
