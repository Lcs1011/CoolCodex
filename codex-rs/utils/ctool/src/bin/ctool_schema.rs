use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let specs = ctool::registry::available_specs();
    let json = serde_json::to_string_pretty(&specs)?;

    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None => {
            println!("{json}");
            Ok(())
        }
        Some("--check") => {
            println!("CTool schema export OK: {} tools", specs.len());
            Ok(())
        }
        Some("--out") => {
            let Some(path) = args.next() else {
                return Err(invalid_input("--out requires a target path").into());
            };
            if args.next().is_some() {
                return Err(invalid_input("--out accepts exactly one target path").into());
            }

            let path = PathBuf::from(path);
            if let Some(parent) = path.parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, json)?;
            println!(
                "CTool schema exported: {} tools -> {}",
                specs.len(),
                path.display()
            );
            Ok(())
        }
        Some(other) => Err(invalid_input(format!(
            "unknown argument: {other}\nusage:\n  ctool_schema\n  ctool_schema --check\n  ctool_schema --out <path>"
        ))
        .into()),
    }
}

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}
