use std::env;
use std::process::ExitCode;

mod system;
use system::Emulator;

fn usage(progname: &str) {
    eprintln!("Usage: {} <romfile>", progname);
}

fn main() -> ExitCode {
    println!(concat!(
        "RustGB - A Game Boy Emulator\n",
        "~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
    ));

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        usage(&args[0]);
        return ExitCode::from(2);
    }

    let romfile = &args[1];

    println!("ROM file: {}", romfile);

    let mut gb = Emulator::new();
    if let Err(err) = gb.load_rom(romfile) {
        eprintln!("Error: Could not load ROM file: {}", err);
        return ExitCode::FAILURE;
    }

    println!("ROM file loaded (size={} bytes)", gb.get_rom_size());

    ExitCode::SUCCESS
}
