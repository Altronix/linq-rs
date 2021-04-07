extern crate futures;
extern crate linq;

/// Logging dependencies
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

///
use futures::executor::block_on;
use log::info;
use slog::Drain;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let _log = slog::Logger::root(drain, o!());
    let _scope_guard = slog_scope::set_global_logger(_log);
    let _log_guard = slog_stdlog::init().unwrap();

    info!("{}", linq::io::Io::version());

    let mut l = linq::io::Io::new();
    let future = async {
        let result = l.scan().await.unwrap();
        let result = l.get(&result[0].serial, "/ATX/network").await.unwrap();
        info!("{}", result);
    };
    block_on(future);
    l.close().unwrap();
}
