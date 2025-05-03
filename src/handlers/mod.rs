pub mod cmd;
pub mod callback;
pub mod start;

// Реэкспортируем все публичные элементы из модуля start
pub use cmd::*;
pub use start::handle_callback_query;
