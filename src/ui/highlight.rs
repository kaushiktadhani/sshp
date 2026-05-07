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

#[cfg(test)]
mod tests {
    use super::*;

    fn render_to_string(text: &str, query: &str) -> String {
        let mut buf: Vec<u8> = Vec::new();
        render_host_with_highlight(&mut buf, text, query).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn test_empty_query_returns_plain_text() {
        let result = render_to_string("production-server", "");
        assert_eq!(result, "production-server");
    }

    #[test]
    fn test_no_match_returns_plain_text() {
        let result = render_to_string("production-server", "xyz");
        assert_eq!(result, "production-server");
    }

    #[test]
    fn test_match_contains_query_text() {
        let result = render_to_string("production-server", "prod");
        assert!(result.contains("prod"));
    }

    #[test]
    fn test_case_insensitive_match() {
        let result = render_to_string("Production-Server", "prod");
        assert!(result.contains("Prod"));
    }

    #[test]
    fn test_multiple_matches() {
        let result = render_to_string("server-server", "server");
        // Both occurrences of "server" should be in the output
        let count = result.matches("server").count();
        assert_eq!(count, 2);
    }
}
