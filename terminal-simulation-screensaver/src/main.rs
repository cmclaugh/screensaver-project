use life::LifeSimulator;
use std::{time::Duration, error::Error};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self,Event,KeyCode},
        execute,
        terminal,
    },
    Terminal,
};

mod life;

fn main() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let result = run_app(&mut terminal);

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;

    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let height = terminal.get_frame().size().height;
    let width = terminal.get_frame().size().width;
    let mut simulator = LifeSimulator::new(height.into(), width.into());
    loop {
        terminal.draw(|frame| frame.render_widget(&simulator,frame.size()))?;
        if event::poll(Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == event::KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        _ => (),
                    }
                },
                _ => (),
            }
        }
        simulator.update();
    }

    Ok(())
}