/*
This can get complicated, thus why this is
in a seperate file
*/

use cpal::{
    traits::{HostTrait, DeviceTrait, StreamTrait},
    StreamError
};
use pyo3::{
    pyfunction,
    PyResult
};
use std::{
    thread,
    time::Duration,
    sync::{Arc, Mutex},
    io::BufWriter,
    fs::File
};


// https://github.com/RustAudio/cpal/blob/b1457e5945cfeb136b7b04c8870b63c2ae3f2212/examples/record_wav.rs#L142-L148
fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    match format {
        cpal::SampleFormat::U16 => hound::SampleFormat::Int,
        cpal::SampleFormat::I16 => hound::SampleFormat::Int,
        cpal::SampleFormat::F32 => hound::SampleFormat::Float,
    }
}

// https://github.com/RustAudio/cpal/blob/b1457e5945cfeb136b7b04c8870b63c2ae3f2212/examples/record_wav.rs#L150-L157
fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

// https://github.com/RustAudio/cpal/blob/b1457e5945cfeb136b7b04c8870b63c2ae3f2212/examples/record_wav.rs#L159
type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = cpal::Sample::from(&sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

#[pyfunction]
pub fn record_audio(fp: String, seconds: u64) -> PyResult<()> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .expect("No input device");

    let supported_stream_config = device.default_input_config()
        .expect("Failed to build default input config");
    let config = supported_stream_config.config();

    // https://github.com/RustAudio/cpal/blob/master/examples/record_wav.rs#L100-L130

    let wav_spec = wav_spec_from_config(&supported_stream_config);

    let writer = hound::WavWriter::create(fp, wav_spec)
        .expect("Failed to create WavWriter");
    let writer = Arc::new(Mutex::new(Some(writer)));
    let writer_2 = writer; // clone occurs here

    let err_fn = move |error: StreamError| {
        eprintln!("Error while recording: {}", error);
    };

    println!("Started to record...");

    let stream = match supported_stream_config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
            err_fn,
        ).expect("Failed to build input stream"),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config,
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
        ).expect("Failed to build input stream"),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &config,
            move |data, _: &_| write_input_data::<u16, i16>(data, &writer_2),
            err_fn,
        ).expect("Failed to build input stream"),
    };

    stream.play()
        .unwrap();
    
    thread::sleep(Duration::from_secs(seconds));


    Ok(())
}