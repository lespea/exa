use ansi_term::Style;
use datetime::TimeZone;

use crate::output::cell::TextCell;
use crate::output::time::TimeFormat;

pub trait Render {
    fn render(self, style: Style, tz: &Option<TimeZone>, format: &TimeFormat) -> TextCell;
}

impl Render for Option<std::time::SystemTime> {
    fn render(self, style: Style, tz: &Option<TimeZone>, format: &TimeFormat) -> TextCell {
        let datestamp = if let Some(time) = self {
            if let Some(ref tz) = tz {
                format.format_zoned(time, tz)
            } else {
                format.format_local(time)
            }
        } else {
            String::from("-")
        };

        TextCell::paint(style, datestamp)
    }
}
