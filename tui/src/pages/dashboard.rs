use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    // Dashboard rendern

    let portfolio_value = Paragraph::new(format!(
        "{} {}",
        app.portfolio.value.round_dp(2),
        app.settings.default.currency
    ));

    frame.render_widget(portfolio_value, area);
}
