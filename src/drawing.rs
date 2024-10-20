use {
    crate::*,
    anyhow::Result,
    termimad::crossterm::{
        cursor,
        execute,
        terminal,
    },
};

/// Move the curstor to the start of the provided line
pub fn goto(
    w: &mut W,
    y: u16,
) -> Result<()> {
    execute!(w, cursor::MoveTo(0, y))?;
    Ok(())
}

/// Clear from the current position to the end of the line
pub fn clear_line(w: &mut W) -> Result<()> {
    execute!(w, terminal::Clear(terminal::ClearType::UntilNewLine))?;
    Ok(())
}
