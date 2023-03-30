use std::sync::Arc;

use exec::Command;
use tokio::sync::RwLock;

use crate::{Greeter, Mode};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerOption {
  Shutdown,
  Reboot,
}

pub fn power(greeter: &mut Greeter, option: PowerOption) {
  let command = match greeter.power_commands.get(&option) {
    None => {
      let mut command = Command::new("shutdown");

      match option {
        PowerOption::Shutdown => command.arg("-h"),
        PowerOption::Reboot => command.arg("-r"),
      };

      command.arg("now");
      command
    }

    Some(args) => {
      let command = match greeter.power_setsid {
        true => {
          let mut command = Command::new("setsid");
          command.args(&args.split(' ').collect::<Vec<&str>>());
          command
        }

        false => {
          let args: Vec<&str> = args.split(' ').collect();

          let mut command = Command::new(args[0]);
          command.args(&args[1..]);
          command
        }
      };

      command
    }
  };

  greeter.power_command = Some(command);
  greeter.notifier.notify_one();
}

pub async fn run(greeter: &Arc<RwLock<Greeter>>, command: &mut Command) {
  greeter.write().await.mode = Mode::Processing;

  let err = command.exec();
  let message = format!("{}: {err}", fl!("command_failed"));

  let mode = greeter.read().await.previous_mode;

  let mut greeter = greeter.write().await;

  greeter.mode = mode;
  greeter.message = Some(message);
}
