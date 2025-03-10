use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::seq::SliceRandom;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug, Clone)]
struct Cell {
    letter: Option<String>,
    selected: bool,
}

impl From<&Cell> for Span<'_> {
    fn from(cell: &Cell) -> Self {
        let style = if cell.selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        Span::styled(cell.letter.clone().unwrap_or_default(), style)
    }
}

impl Cell {
    pub fn new(s: String) -> Self {
        Cell {
            letter: Some(s),
            selected: false,
        }
    }
}

#[derive(Debug)]
pub struct App {
    running: bool,
    input: String,
    board: Vec<Vec<Cell>>,
}

// TODO this should all be config or option
const BAG: &str = "AAAAAAAAAABBBCCDDDDDEEEEEEEEEEEEEEEFFFGGGGHHIIIIIIIIIIJKLLLLLLMMNNNNNNNOOOOOOOOOOPPQQRRRRRRRSSSSTTTTTTTUUUUVVWWXXYYZZ";
const ROWS: usize = 13;
const COLS: usize = 9;

impl App {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut letter_bag = Vec::from(BAG);
        letter_bag.shuffle(&mut rng);
        let mut board: Vec<Vec<Cell>> = letter_bag
            .chunks(COLS)
            .take(ROWS)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|&byte| Cell::new(String::from_utf8(vec![byte]).expect("invalid utf8")))
                    .collect()
            })
            .collect();
        // board[0][0] = Cell {
        //     letter: Some("F".to_owned()),
        //     selected: true,
        // };

        Self {
            input: String::new(),
            running: false,
            board,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let rows: Vec<Line> = self
            .board
            .iter()
            .map(|row| Line::from_iter(row.iter().map(Span::from)))
            .collect();

        let block = Block::default().borders(Borders::ALL).title("Spelltuier");

        let paragraph = Paragraph::new(rows)
            .block(block)
            .alignment(Alignment::Center);

        let area = frame.area();

        let cols_u16 = COLS as u16 + 2;
        let rows_u16 = ROWS as u16 + 2;

        let centered_rect = Rect::new(
            ((area.width.saturating_sub(cols_u16)) / 2) + area.x,
            ((area.height.saturating_sub(rows_u16)) / 2) + area.y,
            cols_u16.min(area.width),
            rows_u16.min(area.height),
        );

        frame.render_widget(paragraph, centered_rect);
    }

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

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
