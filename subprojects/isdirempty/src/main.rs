use std::process::ExitCode;

fn main() -> Result<(), ExitCode> {
    if std::fs::read_dir(std::env::args().nth(1).ok_or(ExitCode::FAILURE)?)
        .or(Err(ExitCode::FAILURE))?
        .next()
        .is_some()
    {
        return Ok(());
    } else {
        return Err(ExitCode::FAILURE);
    }
}
