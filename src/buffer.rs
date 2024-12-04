use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub struct BufferedChannel;

impl BufferedChannel {
    /// Creates a new buffered channel with a specified buffer size and frame rate target.
    pub fn new<T: Send + 'static>(
        buffer_size: usize,
        target_frame_rate: f64,
        tx: impl Fn(T) + Send + Sync + 'static,
    ) -> mpsc::Sender<T> {
        let (producer_sender, producer_receiver) = mpsc::channel::<T>();

        thread::spawn(move || {
            let mut buffer = Vec::with_capacity(buffer_size);
            let target_frame_duration = Duration::from_secs_f64(1.0 / target_frame_rate);
            let mut last_frame_time = Instant::now();
            let mut dynamic_frame_duration = target_frame_duration;
            let mut filled = false;

            for item in producer_receiver {
                buffer.push(item);

                if filled || buffer.len() >= buffer_size {
                    if !filled {
                        // Do not count filling the initial buffer
                        last_frame_time = Instant::now();
                    }
                    filled = true;
                    for buffered_item in buffer.drain(..) {
                        let now = Instant::now();
                        let elapsed = now.duration_since(last_frame_time);

                        // Adjust the dynamic frame duration based on how long it took to process the last frame
                        if elapsed < dynamic_frame_duration {
                            // Frame processed faster than target duration, delay the next frame
                            let sleep_duration = dynamic_frame_duration - elapsed;
                            thread::sleep(sleep_duration);
                        } else {
                            // Frame processed slower than target duration, adaptively increase the duration
                            dynamic_frame_duration = dynamic_frame_duration.mul_f32(1.5)
                        }

                        tx(buffered_item);
                        last_frame_time = Instant::now();
                    }
                }
            }
        });

        producer_sender
    }
}
