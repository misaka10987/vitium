use std::{backtrace::Backtrace, panic::PanicHookInfo, sync::atomic::AtomicBool};

use colored::Colorize;

static CRASHED: AtomicBool = AtomicBool::new(false);

pub fn crashed() -> bool {
    CRASHED.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn crash(info: &PanicHookInfo<'_>) {
    eprintln!("{} {info}", "fatal:".red().bold());
    #[cfg(debug_assertions)]
    {
        let trace = Backtrace::force_capture();
        eprintln!("at");
        eprintln!("{trace}");
    }
    if !crashed() {
        CRASHED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    shutup::ROOT.shut();
}
