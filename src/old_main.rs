use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

#[derive(Debug)]
enum BeeType {
    Honey,
    Rock,
}

#[derive(Debug)]
struct QueenBee<'a> {
    harvest_rate: i8,
    r#type: BeeType,
    name: &'a str,
}

#[derive(Debug)]
struct BeeHive<'a> {
    queen_bee: QueenBee<'a>,
    total_bees: i32,
}

const TOTAL_HONEY: i32 = 0;
const HONEY_QUEEN_BEE: QueenBee = QueenBee {
    r#type: BeeType::Honey,
    harvest_rate: 8,
    name: "Big Honey",
};

const ROCK_QUEEN_BEE: QueenBee = QueenBee {
    r#type: BeeType::Rock,
    harvest_rate: 4,
    name: "The Rock",
};

const ALL_BEES: [QueenBee; 2] = [HONEY_QUEEN_BEE, ROCK_QUEEN_BEE];

const HONEY_BEE_HIVE: BeeHive = BeeHive {
    queen_bee: HONEY_QUEEN_BEE,
    total_bees: 0,
};

fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("setup failed")?;
    run(&mut terminal).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

/// Setup the terminal. This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
/// the cursor.
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(crate::ui)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(45),
                Constraint::Percentage(45),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let items: Vec<ListItem> = ALL_BEES
        .map(|bee| {
            ListItem::new(vec![
                Line::from(vec![
                    Span::raw(symbols::DOT),
                    Span::styled(
                        format!("{:?}", bee.name),
                        Style::default()
                            .fg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw(symbols::line::BOTTOM_LEFT),
                    Span::styled(
                        format!("Type: {:?}", bee.r#type),
                        Style::default().fg(Color::LightGreen),
                    ),
                ]),
                Line::from(vec![
                    Span::raw(symbols::line::BOTTOM_LEFT),
                    Span::styled(
                        format!("Harvest rate: {:?}", bee.harvest_rate),
                        Style::default().fg(Color::LightCyan),
                    ),
                ]),
            ])
        })
        .to_vec();
    let block = Block::default()
        .title("All Queen Bees")
        .borders(Borders::ALL);
    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_widget(list, chunks[0]);
    let block = Block::default().title("Beehives").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    let block = Block::default().title("Help").borders(Borders::ALL);
    let paragraph = Paragraph::new("Press 'q' to quit.").block(block);
    f.render_widget(paragraph, chunks[2]);
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
