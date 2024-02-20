use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::Frame;

pub fn draw_popup(frame: &mut Frame, topic: &str) {
    let block = Block::bordered()
        .border_style(Style::new().fg(Color::Red))
        .title_alignment(Alignment::Center)
        .title("Clean retained topics");
    let text = vec![
        Line::raw("Clean the following topic and all relative below?"),
        Line::styled(
            topic,
            Style::new().add_modifier(Modifier::BOLD | Modifier::ITALIC),
        ),
        Line::raw(""),
        Line::raw("Confirm with Enter, abort with Esc"),
    ];
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    let area = popup_area(frame.size());
    frame.render_widget(Clear, area); // clear the background of the popup
    frame.render_widget(paragraph, area);
}

/// helper function to create a centered area using up certain percentage of the available `area`.
fn popup_area(area: Rect) -> Rect {
    let height = 6;
    // The order is important here. Clamp just panics on min > max which is not what is wanted.
    #[allow(clippy::manual_clamp)]
    let width = (area.width.saturating_mul(4) / 5)
        .max(60)
        .min(area.width.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;
    Rect::new(x, y, width, height)
}
