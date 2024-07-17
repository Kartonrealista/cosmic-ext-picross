// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, button, container, menu, mouse_area, text, Grid, Row, Text};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Apply, Element, Renderer, Theme};
use game::{pair_to_index, Board, Game, Tile};
use widget_colors::{blacktheme, red1theme};

mod game;
mod widget_colors;

const REPOSITORY: &str = "https://github.com/edfloreshz/cosmic-app-template";

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct YourApp {
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
impl Application for YourApp {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.example.CosmicAppTemplate";

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
        let mut app = YourApp {
            core,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
            game: Game {
                board: Board::new(10, 10, 30),
            },
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
        playfield(&self.game)
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
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
            Message::GotoMenu => todo!(),
            Message::Reset => todo!(),
            Message::Reveal(id) => self.game.board.board_vec[id].hidden = false,
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

impl YourApp {
    /// The about page for this app.
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../res/icons/hicolor/128x128/apps/com.example.CosmicAppTemplate.svg")
                [..],
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
    let tilebutton = |id: usize| {
        mouse_area(
            match game.board.board_vec[id] {
                Tile {
                    hidden: true,
                    empty: true,
                } => container("").style(theme::Container::Secondary),
                Tile {
                    hidden: true,
                    empty: false,
                } => container("").style(theme::Container::Secondary),
                Tile {
                    hidden: false,
                    empty: true,
                } => container("").style(theme::Container::custom(red1theme)),
                Tile {
                    hidden: false,
                    empty: false,
                } => container("").style(theme::Container::custom(blacktheme)),
            }
            .center_x()
            .center_y()
            .height(50)
            .width(50),
        )
        .on_press(Message::Reveal(id))
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
    container(
        widget::column()
            .push(
                container(playboard.row_spacing(2).row_alignment(Alignment::Center))
                    .style(theme::Container::Primary)
                    .width((52 * game.board.width + 2) as f32)
                    .height((52 * game.board.height + 2) as f32)
                    .center_x()
                    .center_y()
                    .padding(0),
            )
            .push(
                widget::row()
                    .push(menu_button)
                    .push(reset_button)
                    .padding(20)
                    .spacing(20),
            )
            .align_items(Alignment::Center),
    )
    .padding(20)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
}

fn centralize_tile_content(tile_content: Text<Theme, Renderer>) -> Text<Theme, Renderer> {
    tile_content
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
}
