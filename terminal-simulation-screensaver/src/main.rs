use life::LifeSimulator;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode},
        execute, terminal,
    },
    Terminal,
};
use std::{error::Error, time::Duration};

mod life;

fn main() -> Result<(), Box<dyn Error>> {
    // sets up terminal and generates a Terminal object with crossterm
    // boilerplate ratatui code
    terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    // runs the app (and stores result)
    let result = run_app(&mut terminal);

    // resets the terminal after the app exits
    // boilerplate ratatui code
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;

    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    // retrieves height and width of terminal and generates a LifeSimulator of that size
    let height = terminal.get_frame().size().height;
    let width = terminal.get_frame().size().width;
    let mut simulator = LifeSimulator::new(height.into(), width.into());

    // main execution loop
    loop {
        // renders the simulator over the whole terminal
        terminal.draw(|frame| frame.render_widget(&simulator, frame.size()))?;

        // polls for crossterm events for 500ms
        // this is what handles key presses
        // this also functions as the interval for simulator updates
        if event::poll(Duration::from_millis(500))? {
            // breaks the loop and exits the program if 'q' was pressed
            match event::read()? {
                Event::Key(key_event) if key_event.kind == event::KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        _ => (),
                    }
                }
                Event::Resize(new_width, new_height) => {
                    simulator.resize(new_height as usize, new_width as usize);
                }
                _ => (),
            }
        }

        // update simulator and continue loop
        simulator.update();
    }

    Ok(())
}
