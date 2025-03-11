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
        let board: Vec<Vec<Cell>> = letter_bag
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

        // Create the main game block
        let block = Block::default().borders(Borders::ALL).title("Spelltuier");
        let paragraph = Paragraph::new(rows)
            .block(block)
            .alignment(Alignment::Center);

        // Calculate the centered area for the game board
        let area = frame.area();
        let cols_u16 = COLS as u16 + 2;
        let rows_u16 = ROWS as u16 + 2;
        let board_rect = Rect::new(
            ((area.width.saturating_sub(cols_u16)) / 2) + area.x,
            ((area.height.saturating_sub(rows_u16)) / 2) + area.y,
            cols_u16.min(area.width),
            rows_u16.min(area.height),
        );

        // Create the input field
        let input_text = Paragraph::new(self.input.as_str())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Input"))
            .alignment(Alignment::Center);

        // Calculate the input field position (centered horizontally, above the board)
        let input_width = cols_u16.min(area.width);
        let input_height = 3; // Height for the input box (1 for content + 2 for borders)
        let input_rect = Rect::new(
            board_rect.x,
            board_rect.y.saturating_sub(input_height + 1), // Position it above the board with a small gap
            input_width,
            input_height,
        );

        // Render both widgets
        frame.render_widget(input_text, input_rect);
        frame.render_widget(paragraph, board_rect);
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
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char(' ')) => {} // space is no op
            (_, KeyCode::Char(c)) => self.on_letter(c),
            _ => {}
        }
    }

    fn on_letter(&mut self, c: char) {
        self.input += &c.to_string();
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
