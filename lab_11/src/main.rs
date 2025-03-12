pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}



// TODO: Добавьте определение и реализацию Filter.
// Определение структуры Filter
// Структура-обёртка для логгера, фильтрующая сообщения с помощью замыкания.
struct Filter<L, F>
{
    inner: L,      // Внутренний логер
    predicate: F,  // Замыкание для фильтрации сообщений
}

impl<L, F> Filter<L, F>
where
    L: Logger,
    F: Fn(u8, &str) -> bool,
{
    // Создаёт новый фильтрующий логер
    fn new(inner: L, predicate: F) -> Self {
        Self { inner, predicate }
    }
}

impl<L, F> Logger for Filter<L, F>
where
    L: Logger,
    F: Fn(u8, &str) -> bool,
{
    // Логирует сообщение, если оно проходит через фильтр
    fn log(&self, verbosity: u8, message: &str) {
        if (self.predicate)(verbosity, message) {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}
