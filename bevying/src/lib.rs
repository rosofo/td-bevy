pub mod app;
mod double_buffer;
pub mod systems;
use std::{
    borrow::Cow,
    sync::Arc,
    thread::{spawn, JoinHandle},
};

use anyhow::anyhow;
use app::create_app;
use bevy::app::AppExit;
use crossbeam_channel::{bounded, Receiver, Sender};
use double_buffer::DoubleBuffer;
use numpy::{PyArray1, PyArray3, PyArrayMethods};
use pyo3::prelude::*;
use systems::td_events::{Name, TDEventChannels, TDEventChannelsBundle, TDEventsPlugin};
use tracing::{debug, span};

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub enum TDEvent {
    TimeFactor(f32),
}

#[pyclass]
struct Bevy {
    handle: Option<JoinHandle<AppExit>>,
    tx: Sender<f32>,
    rx_out: Receiver<f32>,
    image: Arc<DoubleBuffer>,
    events: (Sender<TDEvent>, Receiver<TDEvent>),
}

#[pymethods]
impl Bevy {
    #[new]
    fn new() -> Self {
        let (tx, rx) = bounded(100);
        let (tx_out, rx_out) = bounded(100);
        let (events_in_tx, events_in_rx) = bounded(100);
        let (events_out_tx, events_out_rx) = bounded(100);
        let image = Arc::new(DoubleBuffer::new(720 * 1280 * 4));
        let image_ = Arc::clone(&image);
        let handle = spawn(move || {
            let span = span!(tracing::Level::INFO, "bevy_thread");
            let _enter = span.enter();
            let mut app = create_app(image_, tx_out, rx);
            app.add_plugins(TDEventsPlugin::<TDEvent>::default());
            app.finish();
            app.cleanup();

            app.set_runner(|mut app| loop {
                app.update();
                if let Some(exit) = app.should_exit() {
                    return exit;
                }
            });
            debug!("App created ");
            app.world_mut().spawn(TDEventChannelsBundle {
                name: Name("events".to_string()),
                channels: TDEventChannels {
                    tx: events_out_tx,
                    rx: events_in_rx,
                },
            });
            app.run()
        });
        Self {
            handle: Some(handle),
            tx,
            rx_out,
            image,
            events: (events_in_tx, events_out_rx),
        }
    }

    fn next(&self) -> f32 {
        self.rx_out
            .try_recv()
            .map_err(|e| anyhow!("recv error: {}", e))
            .unwrap()
    }

    fn get_image<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        let arr = PyArray1::from_slice_bound(py, &self.image.read_buffer())
            .reshape((720, 1280, 4))
            .unwrap();
        arr
    }

    fn send(&self, value: f32) {
        self.tx.send(value).unwrap();
    }

    fn trigger(&self, event: TDEvent) {
        self.events.0.try_send(event).unwrap();
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
    m.add_class::<TDEvent>()?;
    Ok(())
}
