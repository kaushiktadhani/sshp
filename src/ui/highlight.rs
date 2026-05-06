use crossterm::{
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, Write};

pub fn render_host_with_highlight(
    stdout: &mut impl Write,
    text: &str,
    query: &str,
) -> io::Result<()> {
    if query.is_empty() {
        queue!(stdout, Print(text))?;
        return Ok(());
    }

    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();
    let mut last_end = 0;

    for (start, _) in text_lower.match_indices(&query_lower) {
        let end = start + query.len();

        queue!(stdout, Print(&text[last_end..start]))?;
        queue!(
            stdout,
            SetForegroundColor(Color::Black),
            SetBackgroundColor(Color::Yellow),
            Print(&text[start..end]),
            ResetColor
        )?;

        last_end = end;
    }

    queue!(stdout, Print(&text[last_end..]))?;
    Ok(())
}
