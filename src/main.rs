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
    search_filter: String,
}

impl State {
    fn filtered_commands(&self) -> Vec<&Command> {
        if self.search_filter.is_empty() {
            self.commands.iter().collect()
        } else {
            self.commands
                .iter()
                .filter(|command| {
                    command.title.to_lowercase().contains(&self.search_filter)
                        || command
                            .command_text
                            .to_lowercase()
                            .contains(&self.search_filter)
                })
                .collect()
        }
    }

    fn select_down(&mut self) {
        self.selected = (self.selected + 1) % self.filtered_commands().len();
    }

    fn select_up(&mut self) {
        if self.selected == 0 {
            self.selected = self.filtered_commands().len() - 1;
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
        })
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
                if let Some(command) = self.filtered_commands().get(self.selected) {
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
                should_render = true;
            }

            Event::Key(Key::Char(c))
                if c.is_ascii_alphabetic() || c.is_ascii_digit() || c.is_whitespace() =>
            {
                self.search_filter.push(c);
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
        let spacer = ((_rows - self.filtered_commands().len() - 2) as f32 / 2.0).ceil();
        let width = _cols / 2;
        print!(
            "{:^width$}{:^width$}",
            "Title".blue().bold(),
            "Command".blue().bold()
        );
        print!("{}", "\n".repeat(spacer as usize));
        self.filtered_commands()
            .iter()
            .enumerate()
            .for_each(|(idx, command)| {
                if idx == self.selected {
                    println!(
                        "{:^width$}-{:^width$}",
                        command.title.to_string().red().bold(),
                        command.command_text.to_string().red().bold()
                    )
                } else {
                    println!("{:^width$}-{:^width$}", command.title, command.command_text)
                }
            });
        print!(
            "{}",
            "\n".repeat(if self.filtered_commands().len() % 2 == 0 {
                (spacer + 1.0) as usize
            } else {
                spacer as usize
            })
        );
        print!(
            "{}{}",
            ">  Filter: ".blue().bold(),
            self.search_filter.cyan().italic()
        );
    }
}
