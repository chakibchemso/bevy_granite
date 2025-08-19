#[macro_export]
macro_rules! log {
    // Full specification: type, level, category, message + args
    ($type:expr, $level:expr, $category:expr, $($arg:tt)+) => {
        $crate::output::log($type, $level, $category, format!($($arg)+));
    };

    // Level, category, message + args (defaults to Game type)
    ($level:expr, $category:expr, $($arg:tt)+) => {
        $crate::output::log($crate::config::LogType::Game, $level, $category, format!($($arg)+));
    };

    // Category, message + args (defaults to Game type, Info level)
    (cat: $category:expr, $($arg:tt)+) => {
        $crate::output::log($crate::config::LogType::Game, $crate::config::LogLevel::Info, $category, format!($($arg)+));
    };

    // Just message + args (defaults: Game type, Info level, Blank category)
    ($($arg:tt)+) => {
        $crate::output::log(
            $crate::config::LogType::Game,
            $crate::config::LogLevel::Info,
            $crate::config::LogCategory::Blank,
            format!($($arg)+)
        );
    };
}
