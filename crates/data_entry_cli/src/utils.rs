use anyhow::Context;
use reals_server_bot_common::types::SetDate;

pub(crate) fn parse_set_date_from_text(text: &str) -> anyhow::Result<SetDate> {
    Ok(dateparser::parse(text)
        .with_context(|| "Unable to parse date")?
        .naive_utc())
}
