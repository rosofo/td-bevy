mod app;
mod systems;
use std::thread::{spawn, JoinHandle};

use anyhow::anyhow;
use app::{create_app, EchoEvent};
use bevy::{
    app::{App, AppExit},
    prelude::Trigger,
};
use kanal::{bounded, Receiver, Sender};
use pyo3::prelude::*;

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
            let mut app = create_app(rx);
            app.world_mut().observe(move |trigger: Trigger<EchoEvent>| {
                tx_out
                    .try_send(trigger.event().0)
                    .or_else(|e| Err(anyhow!("Failed to send event back to TD: {}", e)))
                    .unwrap();
            });
            app.finish();
            app.cleanup();

            app.set_runner(|mut app| loop {
                app.update();
                if let Some(exit) = app.should_exit() {
                    return exit;
                }
            });
            app.run()
        });
        Self { handle, tx, rx_out }
    }

    fn next(&self) -> f32 {
        self.rx_out
            .try_recv()
            .map(|opt| opt.ok_or(Err::<f32, anyhow::Error>(anyhow!("no events"))))
            .or_else(|e| Err(anyhow!("recv error: {}", e)))
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
    m.add_class::<Bevy>()?;
    Ok(())
}
