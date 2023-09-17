use rubato::{FftFixedInOut, Resampler, Sample};
use std::io::prelude::{Read, Seek, Write};

const BYTE_PER_SAMPLE: usize = 8;

pub(crate) fn read_file<R: Read + Seek>(inbuffer: &mut R, channels: usize) -> Vec<Vec<f64>> {
    let mut buffer = vec![0u8; BYTE_PER_SAMPLE];
    let mut wfs = Vec::with_capacity(channels);
    for _chan in 0..channels {
        wfs.push(Vec::new());
    }
    'outer: loop {
        for wf in wfs.iter_mut() {
            let bytes_read = inbuffer.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break 'outer;
            }
            let value = f64::from_le_bytes(buffer.as_slice().try_into().unwrap());
            wf.push(value);
        }
    }
    wfs
}

/// Helper to write all frames to a file
pub(crate) fn write_frames<W: Write + Seek>(
    waves: Vec<Vec<f64>>,
    output: &mut W,
    frames_to_skip: usize,
    frames_to_write: usize,
) {
    let channels = waves.len();
    let end = frames_to_skip + frames_to_write;
    for frame in frames_to_skip..end {
        for wave in waves.iter().take(channels) {
            let value64 = wave[frame];
            let bytes = value64.to_le_bytes();
            output.write_all(&bytes).unwrap();
        }
    }
}

fn append_frames(
    buffers: &mut [impl AsMut<[f64]>],
    additional: &[impl AsRef<[f64]>],
    at: usize,
    nbr_frames: usize,
) {
    buffers
        .iter_mut()
        .zip(additional.iter())
        .for_each(|(b, a)| {
            let n = b.as_mut().len();
            b.as_mut()[at..n.min(at + nbr_frames)].copy_from_slice(&a.as_ref()[..nbr_frames])
        });
}

pub(crate) fn resample_to_16khz_mono(
    input: &[f32],
    input_rate: usize,
    output_rate: usize,
) -> (Vec<Vec<f32>>, usize) {
    // log::trace!("data: {:#?}", input);
    log::info!("input rate = {} output rate = {}", input_rate, output_rate);
    let chunksize = 1024;
    let mut resampler = FftFixedInOut::<f32>::new(input_rate, output_rate, chunksize, 1).unwrap();
    let start = std::time::Instant::now();
    let mut outdata = vec![vec![]; 1];

    let mut pos = 0;
    while let Ok(oslice) = resampler.process(&[&input[pos..]], None) {
        outdata[0].extend_from_slice(&oslice[0]);
        pos += chunksize;
    }

    let duration = start.elapsed();
    log::info!("Resampling took: {:?}", duration);
    log::info!(
        "Processed {} input samples into {} output samples",
        input.len(),
        outdata[0].len()
    );

    return (outdata, resampler.output_delay());
}
