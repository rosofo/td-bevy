pub mod app;
pub mod systems;
use std::{
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
};

use anyhow::anyhow;
use app::create_app;
use bevy::{
    app::AppExit,
    prelude::{Event, Trigger},
};
use crossbeam_queue::ArrayQueue;
use kanal::{bounded, Receiver, Sender};
use pyo3::{
    prelude::*,
    types::{PyFunction, PyList},
};
use systems::td_renderer::TDExportEvent;
use tracing::{debug, level_filters::LevelFilter, span};

#[pyclass]
struct Bevy {
    handle: JoinHandle<AppExit>,
    tx: Sender<f32>,
    rx_out: Receiver<f32>,
    image: Arc<Mutex<Vec<u8>>>,
}

#[pymethods]
impl Bevy {
    #[new]
    fn new() -> Self {
        let (tx, rx) = bounded(100);
        let (tx_out, rx_out) = bounded(100);
        let image = Arc::new(Mutex::new(vec![0; 1280 * 720 * 4]));
        let image_ = Arc::clone(&image);
        let handle = spawn(move || {
            let span = span!(tracing::Level::INFO, "bevy_thread");
            let _enter = span.enter();
            let mut app = create_app(tx_out, rx);
            app.finish();
            app.cleanup();
            app.observe(move |trigger: Trigger<TDExportEvent>| {
                if let Ok(mut guard) = image_.try_lock() {
                    guard.copy_from_slice(trigger.event().0.as_slice());
                }
            });

            app.set_runner(|mut app| loop {
                app.update();
                if let Some(exit) = app.should_exit() {
                    return exit;
                }
            });
            debug!("App created ");
            app.run()
        });
        Self {
            handle,
            tx,
            rx_out,
            image,
        }
    }

    fn next(&self) -> f32 {
        self.rx_out
            .try_recv()
            .map(|opt| opt.ok_or(Err::<f32, anyhow::Error>(anyhow!("no events"))))
            .map_err(|e| anyhow!("recv error: {}", e))
            .unwrap()
            .unwrap()
    }

    fn get_image(&self) -> Vec<u8> {
        self.image.try_lock().unwrap().clone()
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
