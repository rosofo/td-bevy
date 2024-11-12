pub mod app;
pub mod systems;
use std::thread::{spawn, JoinHandle};

use anyhow::anyhow;
use app::create_app;
use bevy::app::AppExit;
use kanal::{bounded, Receiver, Sender};
use pyo3::prelude::*;
use tracing::{debug, level_filters::LevelFilter, span};

#[pyclass]
struct Bevy {
    handle: JoinHandle<AppExit>,
    tx: Sender<f32>,
    rx_out: Receiver<f32>,
}

#[pymethods]
impl Bevy {
    #[new]
    fn new() -> Self {
        let (tx, rx) = bounded(100);
        let (tx_out, rx_out) = bounded(100);
        let handle = spawn(move || {
            let span = span!(tracing::Level::INFO, "bevy_thread");
            let _enter = span.enter();
            let mut app = create_app(tx_out, rx);
            app.finish();
            app.cleanup();

            app.set_runner(|mut app| loop {
                app.update();
                if let Some(exit) = app.should_exit() {
                    return exit;
                }
            });
            debug!("App created ");
            app.run()
        });
        Self { handle, tx, rx_out }
    }

    fn next(&self) -> f32 {
        self.rx_out
            .try_recv()
            .map(|opt| opt.ok_or(Err::<f32, anyhow::Error>(anyhow!("no events"))))
            .map_err(|e| anyhow!("recv error: {}", e))
            .unwrap()
            .unwrap()
    }

    fn send(&self, value: f32) {
        self.tx.send(value).unwrap();
    }
    fn running(&self) -> bool {
        !self.handle.is_finished()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn bevying(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // let appender = tracing_appender::rolling::hourly("logs", "bv.log");
    // tracing_subscriber::fmt()
    //     .with_max_level(LevelFilter::INFO)
    //     .with_writer(appender)
    //     .init();
    m.add_class::<Bevy>()?;
    Ok(())
}
