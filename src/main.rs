use core::fmt;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

use zellij_tile::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Command {
    pub title: String,
    pub command_text: String,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", self.title, self.command_text)
    }
}

#[derive(Default)]
struct State {
    selected: usize,
    commands: Vec<Command>,
    filtered_commands: Vec<Command>,
    search_filter: String,
}

impl State {
    fn update_filtered_commands(&mut self) {
        if self.search_filter.is_empty() {
            self.filtered_commands = self.commands.to_vec();
        } else {
            self.filtered_commands = self
                .commands
                .iter()
                .filter(|&command| {
                    command.title.to_lowercase().contains(&self.search_filter)
                        || command
                            .command_text
                            .to_lowercase()
                            .contains(&self.search_filter)
                })
                .cloned()
                .collect();
            if self.selected > self.filtered_commands.len() {
                self.selected = self.filtered_commands.len() - 1;
            }
        }
    }

    fn select_down(&mut self) {
        self.selected = (self.selected + 1) % self.filtered_commands.len();
    }

    fn select_up(&mut self) {
        if self.selected == 0 {
            self.selected = self.filtered_commands.len() - 1;
            return;
        }
        self.selected -= 1;
    }

    fn parse_commands(&mut self, commands_list: BTreeMap<String, String>) {
        commands_list.into_iter().for_each(|raw_command| {
            self.commands.push(Command {
                title: raw_command.0,
                command_text: raw_command.1,
            })
        });
        self.update_filtered_commands();
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, commands_list: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::Key]);
        self.parse_commands(commands_list);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::Key(Key::Esc | Key::Ctrl('c')) => {
                hide_self();
            }

            Event::Key(Key::Char('\n')) => {
                if let Some(command) = self.filtered_commands.get(self.selected) {
                    let split_command: Vec<String> = command
                        .command_text
                        .split(' ')
                        .map(|s| s.to_string())
                        .collect();
                    hide_self();
                    open_command_pane(CommandToRun {
                        path: PathBuf::from(&split_command[0]),
                        args: split_command[1..].to_vec(),
                        cwd: None,
                    });
                }
                self.search_filter = String::from("");
            }

            Event::Key(Key::Backspace) => {
                self.search_filter.pop();
                self.update_filtered_commands();
                should_render = true;
            }

            Event::Key(Key::Char(c))
                if c.is_ascii_alphabetic() || c.is_ascii_digit() || c.is_whitespace() =>
            {
                self.search_filter.push(c);
                self.update_filtered_commands();
                should_render = true;
            }

            Event::Key(Key::Down) => {
                if !self.commands.is_empty() {
                    self.select_down();
                    should_render = true;
                }
            }
            Event::Key(Key::Up) => {
                if !self.commands.is_empty() {
                    self.select_up();
                    should_render = true;
                }
            }
            _ => (),
        };

        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        let half: i32 = (_rows as i32 - 2) / 2;
        let mut offset = 0;
        let table = self
            .filtered_commands
            .iter()
            .enumerate()
            .filter(|(idx, _command)| {
                if (self.selected as i32) <= half {
                    offset = half - (self.selected as i32);
                };
                (*idx as i32 - self.selected as i32).abs() < half + offset
            })
            .map(|(idx, command)| {
                if idx == self.selected {
                    vec![
                        Text::new(command.title.to_string().red().to_string()).selected(),
                        Text::new(command.command_text.to_string().red().to_string()).selected(),
                    ]
                } else {
                    vec![
                        Text::new(command.title.to_string()),
                        Text::new(command.command_text.to_string()),
                    ]
                }
            })
            .fold(Table::new().add_row(vec!["Title", "Command"]), |acc, x| {
                acc.add_styled_row(x)
            })
            .add_styled_row(vec![Text::new(format!(
                "{}{}",
                ">  Filter: ".blue().bold(),
                self.search_filter.cyan().italic()
            ))]);
        print_table_with_coordinates(table, _cols / 3, 0, Some(_cols), Some(_rows));
    }
}
