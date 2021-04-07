#[macro_export]
macro_rules! lformat {
    ($file:expr, $line:expr, $cat:expr, $rest:expr) => {
        format!(
            "{:>10.10}:{:<4.4} [{:>4.4}] => {}",
            $file, $line, $cat, $rest
        )
    };
}

// https://github.com/rust-lang/rust/issues/35853 (copy pasta)
#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

#[macro_export]
macro_rules! gen_log_helpers {
    ($cat:literal) => {
        with_dollar_sign! {
            ($d:tt) => {
                #[allow(unused_macros)]
                macro_rules! trace {
                    ($fmt:literal, $d($d args:expr),*) => {
                        llog!("TRACE", $cat, $fmt, $d($d args),*);
                    }
                }
                #[allow(unused_macros)]
                macro_rules! error {
                    ($fmt:literal, $d($d args:expr),*) => {
                        llog!("ERROR", $cat, $fmt, $d($d args),*);
                    }
                }
                #[allow(unused_macros)]
                macro_rules! warn {
                    ($fmt:literal, $d($d args:expr),*) => {
                        llog!("WARN", $cat, $fmt, $d($d args),*);
                    }
                }
                #[allow(unused_macros)]
                macro_rules! debug {
                    ($fmt:literal, $d($d args:expr),*) => {
                        llog!("DEBUG", $cat, $fmt, $d($d args),*);
                    }
                }
                #[allow(unused_macros)]
                macro_rules! info {
                    ($fmt:literal, $d($d args:expr),*) => {
                        llog!("INFO", $cat, $fmt, $d($d args),*);
                    }
                }
            }
        }
    };
}

// Psst... I suck at macros
#[macro_export]
macro_rules! llog {
    ("TRACE", $cat:literal, $mesg:literal) => {
        log::trace!("{}", lformat!(file!(), line!(), $cat, $mesg))
    };
    ("TRACE", $cat:literal, $fmt:literal, $($rest:expr),+) => {
        let m = format!($fmt, $($rest),+);
        log::trace!("{}", lformat!(file!(), line!(), $cat, m))
    };
    ("ERROR", $cat:literal, $mesg:literal) => {
        log::error!("{}", lformat!(file!(), line!(), $cat, $mesg))
    };
    ("ERROR", $cat:literal, $fmt:literal, $($rest:expr),+) => {
        let m = format!($fmt, $($rest),+);
        log::error!("{}", lformat!(file!(), line!(), $cat, m))
    };
    ("WARN", $cat:literal, $mesg:literal) => {
        log::warn!("{}", lformat!(file!(), line!(), $cat, $mesg))
    };
    ("WARN", $cat:literal, $fmt:literal, $($rest:expr),+) => {
        let m = format!($fmt, $($rest),+);
        log::warn!("{}", lformat!(file!(), line!(), $cat, m))
    };
    ("DEBUG", $cat:literal, $mesg:literal) => {
        log::debug!("{}", lformat!(file!(), line!(), $cat, $mesg))
    };
    ("DEBUG", $cat:literal, $fmt:literal, $($rest:expr),+) => {
        let m = format!($fmt, $($rest),+);
        log::debug!("{}", lformat!(file!(), line!(), $cat, m))
    };
    ("INFO", $cat:literal, $mesg:literal) => {
        log::info!("{}", lformat!(file!(), line!(), $cat, $mesg))
    };
    ("INFO", $cat:literal, $fmt:literal, $($rest:expr),+) => {
        let m = format!($fmt, $($rest),+);
        log::info!("{}", lformat!(file!(), line!(), $cat, m))
    };
}
