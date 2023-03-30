#[macro_use]
extern crate smart_default;

#[macro_use]
mod macros;

mod greeter;
mod info;
mod ipc;
mod keyboard;
mod power;
mod ui;

use std::{error::Error, io, process::ExitCode, sync::Arc};

use crossterm::{
  event::EventStream,
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use greetd_ipc::Request;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::RwLock;

pub use self::greeter::*;
use self::ipc::Ipc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
  if let Err(error) = run().await {
    if let Some(AuthStatus::Success) = error.downcast_ref::<AuthStatus>() {
      return ExitCode::SUCCESS;
    }

    return ExitCode::FAILURE;
  }

  ExitCode::SUCCESS
}

async fn run() -> Result<(), Box<dyn Error>> {
  let (notifier, greeter) = Greeter::new().await;
  let mut stdout = io::stdout().lock();

  enable_raw_mode()?;
  execute!(stdout, EnterAlternateScreen)?;

  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  terminal.clear()?;

  let mut events = EventStream::new();
  let ipc = Ipc::new();

  if greeter.remember && !greeter.username.is_empty() {
    ipc.send(Request::CreateSession { username: greeter.username.clone() }).await;
  }

  let greeter = Arc::new(RwLock::new(greeter));

  let mut ipc_handler = ipc.clone();
  loop {
    tokio::select! {
      _ = ipc_handler.handle(greeter.clone()) => {}
      _ = notifier.notified() => {
        let mut grtr = greeter.write().await;
        if let Some(status) = grtr.exit {
          return Err(status.into());
        }
        if let Some(ref mut command) = grtr.power_command {
          terminal.clear()?;
          power::run(&greeter, command).await;
        }
      }
      status = keyboard::handle(greeter.clone(), &mut events, ipc.clone()) => {
        if let Err(status) = status {
          return Err(status.into());
        }
      }
    }

    ui::draw(greeter.clone(), &mut terminal).await?;
  }
}

pub async fn exit(greeter: &mut Greeter, status: AuthStatus) {
  match status {
    AuthStatus::Success => {}
    AuthStatus::Cancel | AuthStatus::Failure => Ipc::cancel(greeter).await,
  }

  let _ = disable_raw_mode();

  greeter.exit = Some(status);
  greeter.notifier.notify_one();
}

#[cfg(debug_assertions)]
pub fn log(msg: &str) {
  use std::io::Write;

  let time = chrono::Utc::now();

  let mut file = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/tuigreet.log").unwrap();
  file.write_all(format!("{:?} - ", time).as_ref()).unwrap();
  file.write_all(msg.as_ref()).unwrap();
  file.write_all("\n".as_bytes()).unwrap();
}
