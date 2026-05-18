use anyhow::Result;
use crossterm::event::{Event, KeyCode, read};
use crossterm::style::{Color, Stylize};
use crossterm::{cursor, execute, terminal};
use std::io::{Stdout, stdout};

#[derive(Debug)]
enum Mode {
    Normal,
    Insert,
}
impl Mode {
    fn into_uppcase(&self) -> String {
        match self {
            Mode::Normal => "normal".to_uppercase(),
            Mode::Insert => "insert".to_uppercase(),
        }
    }
}

enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    Quit,

    EnterMode(Mode),
}

#[derive(Debug)]
pub struct Editor {
    stdout: Stdout,
    mode: Mode,
    cx: u16,
    cy: u16,
    size: (u16, u16),
}
impl Editor {
    pub fn new() -> Self {
        Editor {
            stdout: stdout(),
            mode: Mode::Normal,
            cx: 0,
            cy: 0,
            size: terminal::size().unwrap(),
        }
    }
    fn handle_event(&self, ev: Event) -> Result<Option<Action>> {
        match self.mode {
            Mode::Normal => self.handle_normal_event(ev),
            Mode::Insert => self.handle_insert_event(ev),
        }
    }
    fn handle_normal_event(&self, ev: Event) -> Result<Option<Action>> {
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => return Ok(Some(Action::MoveUp)),
                KeyCode::Down | KeyCode::Char('j') => return Ok(Some(Action::MoveDown)),
                KeyCode::Left | KeyCode::Char('l') => return Ok(Some(Action::MoveRight)),
                KeyCode::Right | KeyCode::Char('h') => return Ok(Some(Action::MoveLeft)),
                KeyCode::Char('q') => return Ok(Some(Action::Quit)),
                KeyCode::Char('i') => return Ok(Some(Action::EnterMode(Mode::Insert))),
                _ => return Ok(None),
            };
        };
        Ok(None)
    }
    fn handle_insert_event(&self, ev: Event) -> Result<Option<Action>> {
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Esc => return Ok(Some(Action::EnterMode(Mode::Normal))),
                _ => return Ok(None),
            };
        };
        Ok(None)
    }
    fn draw(&self) -> Result<()> {
        self.draw_status()?;
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(self.cx, self.cy))?;
        Ok(())
    }
    fn draw_status(&self) -> Result<()> {
        let mut stdout = stdout();
        let mode = self
            .mode
            .into_uppcase()
            .with(Color::White)
            .on(Color::Blue)
            .bold();
        execute!(stdout, cursor::MoveTo(0, self.size.1 - 2))?;
        execute!(stdout, crossterm::style::PrintStyledContent(mode))?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        terminal::enable_raw_mode()?;
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(self.stdout, terminal::EnterAlternateScreen)?;
        loop {
            let ev = read()?;
            if let Some(action) = self.handle_event(ev).unwrap() {
                match action {
                    Action::MoveUp => self.cy = self.cy.saturating_sub(1),
                    Action::MoveDown => self.cy = self.cy.saturating_add(1),
                    Action::MoveRight => self.cx = self.cx.saturating_add(1),
                    Action::MoveLeft => self.cx = self.cx.saturating_sub(1),
                    Action::EnterMode(new_mode) => self.mode = new_mode,
                    Action::Quit => break,
                }
            }
            let _ = self.draw();
        }
        execute!(self.stdout, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
