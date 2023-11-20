use clap::Parser;
use image::io::Reader as ImageReader;
use crate::converter::ToAsciiArt;

use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

mod converter;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
pub struct Args {
    /// The path to an image file
    #[arg(long, default_value = "")]
    file: String,
    /// The width of the ASCII art
    #[arg(long, default_value = "80")]
    width: u32,
    /// The height of the ASCII art
    #[arg(long, default_value = "50")]
    height: u32,
    /// The gamma of the ASCII art
    #[arg(long, default_value = "1.0")]
    gamma: f32,
    /// Whether or not to live edit the ASCII art
    #[arg(long, default_value = "false")]
    live: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.live {
        true => {
            let file = args.file;

            if !std::path::Path::new(&file).exists() {
                return Err("File does not exist".into());
            }

            let result = App::run(file);

            println!("{}", result.unwrap());

            Ok(())
        },
        false => {
            let open_file = ImageReader::open(args.file).unwrap();
            let image = open_file.decode().unwrap();
            let converter = converter::ImageConverter::new(image);
            let options = converter::AsciiOptions::new(args.width, args.height, args.gamma);
            let art = converter.to_ascii_art(Some(options));

            println!("{}", art);

            Ok(())
        }
    }
}

struct App {
    art: String,
    width: u32,
    height: u32,
    gamma: f32,
    selected_field: Fields,
}

#[derive(PartialEq)]
enum Fields {
    Width,
    Height,
    Gamma,
    Finish
}

impl App {
    fn new() -> App {
        App {
            art: String::new(),
            width: 80,
            height: 50,
            gamma: 1.0,
            selected_field: Fields::Width,
        }
    }

    pub fn run(file: String) -> io::Result<String> {
        let mut terminal = init_terminal()?;
        let mut app = App::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(33);

        let open_file = ImageReader::open(file).unwrap();
        let image = open_file.decode().unwrap();
        let converter = converter::ImageConverter::new(image);

        loop {
            let options = converter::AsciiOptions::new(app.width, app.height, app.gamma);
            app.art = converter.to_ascii_art(Some(options));

            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Right => {
                                match app.selected_field {
                                    Fields::Width => {
                                        app.width += 1;
                                    },
                                    Fields::Height => {
                                        app.height += 1;
                                    },
                                    Fields::Gamma => {
                                        app.gamma += 0.1;
                                    },
                                    Fields::Finish => {}
                                }
                            },
                            KeyCode::Left => {
                                match app.selected_field {
                                    Fields::Width => {
                                        app.width -= 1;
                                    },
                                    Fields::Height => {
                                        app.height -= 1;
                                    },
                                    Fields::Gamma => {
                                        app.gamma -= 0.1;
                                    },
                                    Fields::Finish => {}
                                }
                            },
                            KeyCode::Up => {
                                match app.selected_field {
                                    Fields::Width => {
                                        app.selected_field = Fields::Finish;
                                    },
                                    Fields::Height => {
                                        app.selected_field = Fields::Width;
                                    },
                                    Fields::Gamma => {
                                        app.selected_field = Fields::Height;
                                    },
                                    Fields::Finish => {
                                        app.selected_field = Fields::Gamma;
                                    }
                                }
                            },
                            KeyCode::Down => {
                                match app.selected_field {
                                    Fields::Width => {
                                        app.selected_field = Fields::Height;
                                    },
                                    Fields::Height => {
                                        app.selected_field = Fields::Gamma;
                                    },
                                    Fields::Gamma => {
                                        app.selected_field = Fields::Finish;
                                    },
                                    Fields::Finish => {
                                        app.selected_field = Fields::Width;
                                    }
                                }
                            },
                            KeyCode::Enter => {
                                match app.selected_field {
                                    Fields::Finish => {
                                        break;
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }

        let _ = restore_terminal();

        return Ok(app.art);
    }

    fn on_tick(&mut self) {
    
    }

    fn ui(&self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(frame.size());

        frame.render_widget(self.boxes_canvas(main_layout[0]), main_layout[0]);
        frame.render_widget(self.boxes_options(main_layout[1]), main_layout[1]);
    }

    fn boxes_options(&self, area: Rect) -> impl Widget {
        let (left, right, bottom, top) = (0.0, area.width as f64, 0.0, area.height as f64 * 2.0 - 4.0);

        let width = self.width.to_string();
        let width_text = format!("Width: {} {}", width, if self.selected_field == Fields::Width { "<" } else { "" });

        let height = self.height.to_string();
        let height_text = format!("Height: {} {}", height, if self.selected_field == Fields::Height { "<" } else { "" });

        let gamma = self.gamma.to_string();
        let gamma_text = format!("Gamma: {} {}", gamma, if self.selected_field == Fields::Gamma { "<" } else { "" });

        let confirm_text = format!("Confirm {}", if self.selected_field == Fields::Finish { "<" } else { "" });

        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Options"))
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(move |ctx| {
                ctx.draw(&Rectangle {
                    x: left,
                    y: bottom,
                    width: right - left,
                    height: top - bottom,
                    color: Color::White,
                });

                ctx.print(2.0, top - 4.0, width_text.clone());
                ctx.print(2.0, top - 6.0, height_text.clone());
                ctx.print(2.0, top - 8.0, gamma_text.clone());
                ctx.print(2.0, bottom + 1.0, confirm_text.clone());
            })
    }

    fn boxes_canvas(&self, area: Rect) -> impl Widget {
        let (left, right, bottom, top) = (0.0, area.width as f64, 0.0, area.height as f64 * 2.0 - 4.0);

        let art = self.art.clone();

        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Art"))
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(move |ctx| {
                ctx.draw(&Rectangle {
                    x: left,
                    y: bottom,
                    width: right - left,
                    height: top - bottom,
                    color: Color::White,
                });
                let mut x = 1.0;
                let mut y = top - 2.0; 

                for c in art.chars() {
                    if c == '\n' {
                        x = 1.0;
                        y -= 1.0; 
                        continue;
                    }
                    ctx.print(x, y, c.to_string());
                    x += 1.0;
                }
            })
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
