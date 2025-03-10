use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::seq::SliceRandom;
use ratatui::{text::Line, widgets::Paragraph, DefaultTerminal, Frame};

#[derive(Debug, Clone)]
struct Cell {
    letter: Option<u8>,
    selected: bool,
}

#[derive(Debug)]
pub struct App {
    running: bool,
    input: String,
    board: Vec<Vec<String>>,
}

// TODO this should all be config or option
const BAG: &str = "AAAAAAAAABBCCDDDDEEEEEEEEEEEEFFGGGHHIIIIIIIIIJKLLLLMMNNNNNNOOOOOOOOPPQRRRRRRSSSSTTTTTTUUUUVVWWXYYZ";
const ROWS: usize = 10; // TODO: 13 again, but need a bigger bag
const COLS: usize = 9;

impl App {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut letter_bag = Vec::from(BAG);
        letter_bag.shuffle(&mut rng);
        let board: Vec<Vec<String>> = letter_bag
            .chunks(COLS)
            .take(ROWS)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|&byte| String::from_utf8(vec![byte]).unwrap_or_default())
                    .collect()
            })
            .collect();

        Self {
            input: String::new(),
            running: false,
            board,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let rows: Vec<Line> = self
            .board
            .iter()
            .map(|r| Line::from_iter(r.iter().cloned()))
            .collect();
        frame.render_widget(Paragraph::new(rows), frame.area())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
