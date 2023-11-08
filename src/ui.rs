use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{App, MULTIPLIER_COST, PETTING_MACHINE_COST};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let multiplier_buy_text = if app.cat_petted >= MULTIPLIER_COST {
        t!("multiplier_buy_text")
    } else {
        String::new()
    };
    let petting_machine_buy_text = if app.cat_petted >= PETTING_MACHINE_COST {
        t!("petting_machine_buy_text")
    } else {
        String::new()
    };

    frame.render_widget(
        Paragraph::new(t!("ui",
        cat_petted = app.cat_petted,
        multiplier = app.multiplier,
        petting_machine = app.cat_petting_machine,
        multiplier_buy_text = multiplier_buy_text,
        petting_machine_buy_text = petting_machine_buy_text))
        .block(
            Block::default()
                .title(t!("title"))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        frame.size(),
    )
}
