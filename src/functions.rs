#[cfg(windows)]
use crate::sys::winapi::ansi::set_virtual_terminal_processing;

#[cfg(windows)]
pub fn supports_ansi() -> bool {
    // Some terminals on windows like GitBash can't use WinaApi calls directly so when we try to enable the ANSI-flag for windows this won't work.
    // Because of that we should check first if the TERM-variable is set and see if the current terminal is a terminal who does support ANSI.

    if is_specific_term() {
        return true;
    }

    // if it is not listed we should try with WinApi to check if we do support ANSI-codes.
    set_virtual_terminal_processing(true)
        .map(|_| true)
        .unwrap_or(false)
}

// checks if the 'TERM' environment variable is set to check if the terminal supports ANSI-codes.
// I got the list of terminals from here: https://github.com/keqingrong/supports-ansi/blob/master/index.js
#[cfg(windows)]
fn is_specific_term() -> bool {
    const TERMS: [&'static str; 15] = [
        "xterm",  // xterm, PuTTY, Mintty
        "rxvt",   // RXVT
        "eterm",  // Eterm
        "screen", // GNU screen, tmux
        "tmux",   // tmux
        "vt100", "vt102", "vt220", "vt320",   // DEC VT series
        "ansi",    // ANSI
        "scoansi", // SCO ANSI
        "cygwin",  // Cygwin, MinGW
        "linux",   // Linux console
        "konsole", // Konsole
        "bvterm",  // Bitvise SSH Client
    ];

    match std::env::var("TERM") {
        Ok(val) => val != "dumb" || TERMS.contains(&val.as_str()),
        Err(_) => false,
    }
}
