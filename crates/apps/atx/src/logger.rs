use crate::slog::Drain;
use slog;
use slog_async;

struct VerboseFilter<D> {
    drain: D,
    verbose: bool,
}
impl<D> Drain for VerboseFilter<D>
where
    D: Drain,
{
    type Ok = Option<D::Ok>;
    type Err = Option<D::Err>;

    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> std::result::Result<Self::Ok, Self::Err> {
        let level = if self.verbose {
            slog::Level::Debug
        } else {
            slog::Level::Warning
        };
        if record.level().is_at_least(level) {
            self.drain.log(record, values).map(Some).map_err(Some)
        } else {
            Ok(None)
        }
    }
}

pub fn init(verbose: bool) -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = VerboseFilter { drain, verbose }.fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}
