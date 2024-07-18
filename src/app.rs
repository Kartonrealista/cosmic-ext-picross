// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{
    self, button, container, menu, mouse_area, text, text_input, Column, Grid, Row, Text,
};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Apply, Element, Renderer, Theme};
use game::{pair_to_index, Board, Game, Tile, Winstate};
use widget_colors::{blacktheme, gray1theme, gray2theme, whitetheme};

mod game;
mod widget_colors;

const REPOSITORY: &str = "https://github.com/Kartonrealista/cosmic-ext-picross";

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct Picross {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    game: Game,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    LaunchUrl(String),
    ToggleContextPage(ContextPage),
    GotoMenu,
    Reset,
    Reveal(usize),
    Mark(usize),
    InputHeight(String),
    InputWidth(String),
    InputFilledCount(String),
    StartPressed,
}

/// Identifies a context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the async executor that will be used to run your application's commands.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for Picross {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "git.Kartonrealista.cosmic-ext-picross";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Picross {
            core,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
            game: Game::new(),
        };

        let command = app.update_titles();

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        if self.game.menu.start_pressed {
            playfield(&self.game)
        } else {
            menu(&self.game)
        }
        .apply(widget::container)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    /// Application messages are handled here. The application state can be modified based on
    /// what message was received. Commands may be returned for asynchronous execution on a
    /// background thread managed by the application's executor.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::LaunchUrl(url) => {
                let _result = open::that_detached(url);
            }

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }

                // Set the title of the context drawer.
                self.set_context_title(context_page.title());
            }
            Message::Reveal(id) => {
                self.game.board.board_vec[id].hidden = false;
                self.game.wincheck();
            }
            Message::Mark(id) => {
                let marked = &mut self.game.board.board_vec[id].marked;
                *marked = !*marked;
                self.game.wincheck()
            }
            Message::GotoMenu => {
                self.game = Game::new();
            }
            Message::InputWidth(input) => self.game.menu.width_input = input,
            Message::InputHeight(input) => self.game.menu.height_input = input,
            Message::InputFilledCount(input) => self.game.menu.filled_count_input = input,
            Message::StartPressed => {
                self.game.board.width = self.game.menu.width_input.parse().unwrap();
                self.game.board.height = self.game.menu.height_input.parse().unwrap();
                self.game.board.filled_count = self.game.menu.filled_count_input.parse().unwrap();
                self.game.board = Board::new(
                    self.game.board.width,
                    self.game.board.height,
                    self.game.board.filled_count,
                );
                self.game.menu.start_pressed = true;
            }

            Message::Reset => {
                self.game.board = Board::new(
                    self.game.board.width,
                    self.game.board.height,
                    self.game.board.filled_count,
                );
                self.game.winstate = Winstate::InProgress;
            }
        }
        Command::none()
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => self.about(),
        })
    }
}

impl Picross {
    /// The about page for this app.
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!(
                "../res/icons/hicolor/scalable/apps/git.Kartonrealista.cosmic-ext-picross.svg"
            )[..],
        ));

        let title = widget::text::title3(fl!("app-title"));

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_titles(&mut self) -> Command<Message> {
        let window_title = fl!("app-title");
        let header_title = String::new();

        self.set_header_title(header_title);
        self.set_window_title(window_title)
    }
}

fn playfield(game: &Game) -> widget::Container<'_, Message, cosmic::Theme> {
    let disabled_tilebutton = |id: usize| match game.board.board_vec[id] {
        Tile {
            hidden: true,
            marked: true,
            ..
        } => mouse_area(
            container(centralize_tile_content(text(String::from("X")).size(25)))
                .style(theme::Container::Secondary)
                .center_x()
                .center_y()
                .height(50)
                .width(50),
        ),
        Tile {
            hidden: true,
            marked: false,
            ..
        } => mouse_area(
            container("")
                .style(theme::Container::Secondary)
                .center_x()
                .center_y()
                .height(50)
                .width(50),
        ),
        Tile {
            hidden: false,
            empty: true,
            ..
        } => mouse_area(
            container("")
                .style(theme::Container::custom(gray1theme))
                .center_x()
                .center_y()
                .height(50)
                .width(50),
        ),
        Tile {
            hidden: false,
            empty: false,
            ..
        } => mouse_area(
            container("")
                .style(theme::Container::custom(blacktheme))
                .center_x()
                .center_y()
                .height(50)
                .width(50),
        ),
    };
    let tilebutton = |id: usize| match game.winstate {
        Winstate::Won => disabled_tilebutton(id),
        Winstate::Lost => disabled_tilebutton(id),
        Winstate::InProgress => disabled_tilebutton(id)
            .on_press(Message::Reveal(id))
            .on_right_press(Message::Mark(id)),
    };
    let playboard = (0..game.board.height).fold(Grid::new(), |acc, row| {
        let new_row = (0..game.board.width).fold(Row::new(), |acc2, column| {
            acc2.push(tilebutton(pair_to_index(row, column, game.board.width)))
        });
        acc.push(new_row.spacing(2).align_items(Alignment::Center))
            .insert_row()
    });
    let menu_button = button("Menu")
        .on_press(Message::GotoMenu)
        .style(theme::Button::Suggested);
    let reset_button = button("Reset")
        .on_press(Message::Reset)
        .style(theme::Button::Destructive);
    let winstate_text = match game.winstate {
        Winstate::Won => "You won!",
        Winstate::Lost => "You lost!",
        Winstate::InProgress => "Game in progress...",
    };
    let vertical_count_column = |vec: &Vec<usize>| {
        vec.iter()
            .fold(Column::new(), |acc: Column<'_, Message>, count| {
                acc.push(
                    container(centralize_tile_content(text(format!("{}", count))))
                        .height(20)
                        .center_x()
                        .center_y(),
                )
            })
    };
    let vertical_counts = (&game)
        .board
        .vertical_count
        .iter()
        .fold(Row::new(), |acc, column| {
            acc.push(
                container(vertical_count_column(column).align_items(Alignment::Center))
                    .style(theme::Container::Primary)
                    .width(50)
                    .center_x()
                    .center_y(),
            )
        });
    let horizontal_count_row = |vec: &Vec<usize>| {
        vec.iter().fold(Row::new(), |acc: Row<'_, Message>, count| {
            acc.push(
                container(centralize_tile_content(text(format!("{}", count))))
                    .width(20)
                    .center_x()
                    .center_y(),
            )
        })
    };
    let horizontal_counts =
        (&game)
            .board
            .horizontal_count
            .iter()
            .fold(Column::new(), |acc, row| {
                acc.push(
                    container(horizontal_count_row(row).align_items(Alignment::Center))
                        .style(theme::Container::Primary)
                        .height(50)
                        .center_x()
                        .center_y(),
                )
            });

    container(
        widget::column()
            .push(
                container(
                    container(vertical_counts.spacing(2).align_items(Alignment::End))
                        .style(theme::Container::Primary)
                        .height((((game.board.height + 1) / 2) * 20) as u16)
                        .center_x()
                        .align_y(Vertical::Bottom),
                )
                .style(theme::Container::Primary)
                .align_x(Horizontal::Right)
                .width((52 * game.board.width + 2) as f32)
                .center_x()
                .center_y()
                .padding(0),
            )
            .push(
                widget::column()
                    .push(
                        widget::row()
                            .push(
                                container(
                                    container(
                                        horizontal_counts.spacing(2).align_items(Alignment::End),
                                    )
                                    .width((((game.board.width + 1) / 2) * 20) as u16)
                                    .style(theme::Container::Primary)
                                    .align_x(Horizontal::Right)
                                    .center_y(),
                                )
                                .style(theme::Container::Primary)
                                .height((52 * game.board.height + 2) as f32)
                                .center_x()
                                .center_y()
                                .padding(0),
                            )
                            .push(
                                container(
                                    playboard.row_spacing(2).row_alignment(Alignment::Center),
                                )
                                .style(theme::Container::Primary)
                                .width((52 * game.board.width + 2) as f32)
                                .height((52 * game.board.height + 2) as f32)
                                .center_x()
                                .center_y()
                                .padding(0),
                            )
                            .align_items(Alignment::Center),
                    )
                    .push(
                        widget::row()
                            .push(menu_button)
                            .push(reset_button)
                            .padding(20)
                            .spacing(20),
                    )
                    .push(container(text(winstate_text)))
                    .align_items(Alignment::Center),
            )
            .align_items(Alignment::End),
    )
    .padding(20)
    .align_x(Horizontal::Right)
    .align_y(Vertical::Center)
}

fn centralize_tile_content(tile_content: Text<Theme, Renderer>) -> Text<Theme, Renderer> {
    tile_content
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
}
fn menu(game: &Game) -> widget::Container<'_, Message, cosmic::Theme> {
    let width_box = text_input("", &game.menu.width_input).on_input(Message::InputWidth);
    let height_box = text_input("", &game.menu.height_input).on_input(Message::InputHeight);
    let filled_count_box =
        text_input("", &game.menu.filled_count_input).on_input(Message::InputFilledCount);
    let start_game_button = button(centralize_tile_content(text("START")))
        .on_press(Message::StartPressed)
        .style(theme::Button::Suggested)
        .width(130)
        .height(55);
    container(
        widget::column()
            .push(
                widget::row()
                    .push(text("Width: "))
                    .push(width_box.width(40))
                    .align_items(Alignment::Center),
            )
            .push(
                widget::row()
                    .push(text("Height: "))
                    .push(height_box.width(40))
                    .align_items(Alignment::Center),
            )
            .push(
                widget::row()
                    .push(text("Filled boxes: "))
                    .push(filled_count_box.width(40))
                    .align_items(Alignment::Center),
            )
            .push(start_game_button)
            .align_items(Alignment::End)
            .spacing(20),
    )
}
