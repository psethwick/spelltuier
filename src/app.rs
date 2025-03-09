use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::seq::IndexedMutRandom;
use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Row, Table},
    DefaultTerminal, Frame,
};

#[derive(Debug)]
struct Cell {
    letter: Option<u8>,
    selected: bool,
}

#[derive(Debug)]
pub struct App {
    running: bool,
    input: String,
    board: Vec<Vec<Cell>>,
}

// TODO this should all be config or option
const BAG: &str = "AAAAAAAAABBCCDDDDEEEEEEEEEEEEFFGGGHHIIIIIIIIIJKLLLLMMNNNNNNOOOOOOOOPPQRRRRRRSSSSTTTTTTUUUUVVWWXYYZ";
const ROWS: i32 = 13;
const COLS: i32 = 9;

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        // TODO: finish this
        // let mut rng = rand::rng();
        // let mut letter_bag = Vec::from(BAG);
        // let mut board = Vec::new();
        // let c = letter_bag.choose_mut(&mut rng).unwrap();
        // generate right number of rows, columns
        // maybe the config should include distribution? idk, maybe the simple bag is fine
        let board = vec![
            vec!['b'.to_string(), 'b'.to_string()],
            vec!['A'.to_string(), 'A'.to_string()],
        ];

        let rows: Vec<_> = <Vec<Vec<String>> as Clone>::clone(&self.board)
            .into_iter()
            .map(|r| Row::new(r))
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
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        let rows = self
            .board
            .into_iter()
            .map(|r| Row::new(r.to_owned()))
            .collect();
        // frame.render_widget(
        //     Table::new(rows, widths)
        // )
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
