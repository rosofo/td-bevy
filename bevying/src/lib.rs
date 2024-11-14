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
use numpy::{PyArray1, PyArray3, PyArrayMethods, ToPyArray};
use pyo3::{
    prelude::*,
    types::{PyFunction, PyList},
};
use tracing::{debug, level_filters::LevelFilter, span};

#[pyclass]
struct Bevy {
    handle: Option<JoinHandle<AppExit>>,
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
            let mut app = create_app(image_, tx_out, rx);
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
        Self {
            handle: Some(handle),
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

    fn get_image<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        PyArray1::from_slice_bound(py, self.image.try_lock().unwrap().as_slice())
            .reshape((720, 1280, 4))
            .unwrap()
    }

    fn send(&self, value: f32) {
        self.tx.send(value).unwrap();
    }
    fn running(&self) -> bool {
        !self.handle.as_ref().unwrap().is_finished()
    }
    fn status(&mut self) -> String {
        if self.running() {
            "Running".to_string()
        } else {
            match self.handle.take().unwrap().join() {
                Ok(a) => format!("Ok {:?}", a),
                Err(b) => {
                    if let Some(msg) = b.downcast_ref::<&'static str>() {
                        msg.to_string()
                    } else if let Some(msg) = b.downcast_ref::<String>() {
                        msg.clone()
                    } else {
                        format!("?{:?}", b)
                    }
                }
            }
        }
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
