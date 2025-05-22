use std::sync::Arc;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    widgets::{Clear, List, ListItem},
};
use tui_input::{backend::crossterm::EventHandler, Input};
use wiki_api::languages::{Language, LANGUAGES};

use crate::{
    action::{Action, ActionPacket, ActionResult, SearchAction},
    config::{Config, Theme},
    terminal::Frame,
    ui::{centered_rect, StatefulList},
};

use super::{Component, Focus};

pub struct SearchLanguageSelectionComponent {
    input: Input,
    focus: Focus,
    list: StatefulList<Language>,

    config: Arc<Config>,
    theme: Arc<Theme>,
}

impl SearchLanguageSelectionComponent {
    pub fn new(config: Arc<Config>, theme: Arc<Theme>) -> Self {
        Self {
            input: Input::default(),
            list: StatefulList::with_items(Vec::new()),
            focus: Focus::default(),

            config,
            theme,
        }
    }

    fn update_list(&mut self) {
        let input_value = self.input.value();
        let sorted_languages = LANGUAGES
            .iter()
            .filter(|lang| {
                let lang = lang.name().to_lowercase();
                let query = input_value.to_lowercase();
                lang.contains(&query)
            })
            .map(|x| x.to_owned())
            .collect::<Vec<Language>>();
        self.list = StatefulList::with_items(sorted_languages);
    }
}

impl Component for SearchLanguageSelectionComponent {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> ActionResult {
        if self.config.bindings.global.submit.matches_event(key) {
            if let Some(lang) = self.list.selected() {
                let mut packet =
                    ActionPacket::single(Action::SwitchContextSearch).action(Action::PopPopup);

                if self.config.ui.popup_search_language_changed {
                    packet = packet.action(Action::PopupMessage(
                        "Information".to_string(),
                        format!("Changed the language for searches to '{}'", lang.name()),
                    ));
                }

                return packet
                    .action(Action::Search(SearchAction::ChangeLanguage(
                        lang.to_owned(),
                    )))
                    .into();
            }
            return ActionResult::Ignored;
        }

        if self.config.bindings.global.pop_popup.matches_event(key) {
            return Action::PopPopup.into();
        }

        match key.code {
            KeyCode::Tab | KeyCode::BackTab => {
                self.focus = match self.focus {
                    Focus::Input => Focus::List,
                    Focus::List => Focus::Input,
                };

                tracing::debug!("focus now: '{:?}'", self.focus);

                ActionResult::consumed()
            }
            KeyCode::Char('i') if self.focus == Focus::List => {
                self.focus = Focus::Input;
                ActionResult::consumed()
            }

            KeyCode::F(2) => Action::PopPopup.into(),

            _ if self.focus == Focus::Input => {
                self.input.handle_event(&crossterm::event::Event::Key(key));
                self.update_list();
                ActionResult::consumed()
            }
            _ => ActionResult::Ignored,
        }
    }

    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::ScrollUp(n) => {
                for _ in 0..n {
                    self.list.previous()
                }
                ActionResult::consumed()
            }
            Action::ScrollDown(n) => {
                for _ in 0..n {
                    self.list.next()
                }
                ActionResult::consumed()
            }
            Action::UnselectScroll => {
                self.list.unselect();
                ActionResult::consumed()
            }
            _ => ActionResult::Ignored,
        }
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let popup_block = self
            .theme
            .default_block()
            .title("Switch Search Language")
            .style(Style::default().bg(self.theme.bg));
        let area = centered_rect(area, 25, 60);
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let (input_area, list_area) = {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(1), Constraint::Percentage(100)])
                .split(area);
            (chunks[0], chunks[1])
        };

        let scroll = self.input.visual_scroll(input_area.width as usize);
        let cursor = self.input.visual_cursor();
        let value = self.input.value();

        let input_widget = self
            .theme
            .default_paragraph(format!(
                "{}{}",
                value,
                "_".repeat((input_area.width as usize).saturating_sub(value.len()))
            ))
            .scroll((0, scroll as u16));
        f.render_widget(input_widget, input_area);

        if self.focus == Focus::Input {
            f.set_cursor(
                input_area.x + (cursor.max(scroll) - scroll) as u16,
                input_area.y,
            );
        }

        let list_items = self
            .list
            .get_items()
            .iter()
            .map(|x| ListItem::new(x.name().to_owned()).fg(self.theme.fg));
        let list_widget = List::new(list_items).highlight_style(if self.focus == Focus::List {
            Style::default()
                .fg(self.theme.selected_fg)
                .bg(self.theme.selected_bg)
                .add_modifier(Modifier::ITALIC)
        } else {
            Style::default()
        });
        f.render_stateful_widget(list_widget, list_area, self.list.get_state_mut());
    }
}
