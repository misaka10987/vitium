use std::{backtrace::Backtrace, panic::PanicHookInfo, process, sync::atomic::AtomicBool};

use colored::Colorize;

static CRASHED: AtomicBool = AtomicBool::new(false);

pub fn crashed() -> bool {
    CRASHED.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn exit() -> ! {
    process::exit(crashed() as i32)
}

pub fn crash(info: &PanicHookInfo<'_>) {
    eprintln!("{} {info}", "FATAL".red().bold());
    #[cfg(debug_assertions)]
    {
        let trace = Backtrace::force_capture();
        eprintln!("at");
        eprintln!("{trace}");
    }
    if !crashed() {
        CRASHED.store(true, std::sync::atomic::Ordering::Relaxed);
        eprintln!("{}", "SERVER CRASHED".red().bold());
    }
    shutup::ROOT.shut();
}
