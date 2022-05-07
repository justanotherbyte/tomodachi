
use pyo3::prelude::*;
use tts::*;
use std::thread;
use std::time::Duration;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn speak(text: String) -> PyResult<()> {
    let mut tts = Tts::default().unwrap();
    let Features {
        utterance_callbacks,
        ..
    } = tts.supported_features();

    if utterance_callbacks {
        tts.on_utterance_begin(Some(Box::new(|utterance| {
            println!("Started speaking {:?}", utterance)
        }))).unwrap();
        tts.on_utterance_end(Some(Box::new(|utterance| {
            println!("Finished speaking {:?}", utterance)
        }))).unwrap();
        tts.on_utterance_stop(Some(Box::new(|utterance| {
            println!("Stopped speaking {:?}", utterance)
        }))).unwrap();
    }

    tts.set_volume(1.0).unwrap();
    tts.speak(text, true).unwrap();
    
    thread::sleep(Duration::from_secs(3));

    Ok(())
}
/// A Python module implemented in Rust.
#[pymodule]
fn voice(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(speak, m)?)?;
    Ok(())
}