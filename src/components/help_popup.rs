use std::sync::Arc;

use ratatui::{
    layout::{Constraint, Flex::Center, Layout, Rect},
    style::Style,
    widgets::{Clear, Paragraph, Wrap},
};
use tui_input::Input;

use crate::{
    action::{Action, ActionResult},
    config::{Config, Theme},
    terminal::Frame,
    ui::centered_rect,
};

use super::Component;

const BINDINGS_TEXT: &'static str = include_str!("../../docs/docs/configuration/keybindings.md");

pub struct HelpPopupComponent {
    line: u16,

    config: Arc<Config>,
    theme: Arc<Theme>,
}

impl HelpPopupComponent {
    pub fn new(config: Arc<Config>, theme: Arc<Theme> /* , bindings:Keybindings */) -> Self {
        Self {
            line: 0,

            config,
            theme,
        }
    }
}

impl Component for HelpPopupComponent {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> ActionResult {
        if self.config.bindings.global.pop_popup.matches_event(key) {
            return Action::PopPopup.into();
        }

        ActionResult::Ignored
    }

    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::ScrollUp(n) => {
                self.line = if n > self.line { 0 } else { self.line - n };

                ActionResult::consumed()
            }
            Action::ScrollDown(n) => {
                self.line += n;

                ActionResult::consumed()
            }

            _ => ActionResult::Ignored,
        }
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let popup_block = self
            .theme
            .default_block()
            .title("Help")
            .style(Style::default().bg(self.theme.bg));
        let area = centered_rect(area, 95, 95);

        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let [text_area] = Layout::vertical([Constraint::Percentage(100)])
            .margin(1)
            .flex(Center)
            .areas(area);

        let bindings_paragraph = Paragraph::new(BINDINGS_TEXT)
            .scroll((self.line, 0))
            .wrap(Wrap { trim: false });

        f.render_widget(bindings_paragraph, text_area);
    }
}
