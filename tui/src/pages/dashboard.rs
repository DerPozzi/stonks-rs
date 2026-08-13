use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    // Dashboard rendern

    let portfolio_value = Paragraph::new(format!(
        "{} {}",
        app.portfolio_value.round_dp(2),
        app.settings.default.currency.unwrap_or_default()
    ));

    frame.render_widget(portfolio_value, area);
}
