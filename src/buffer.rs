use std::sync::mpsc;
use std::thread;

pub struct BufferedChannel {
}

impl BufferedChannel {
    /// Creates a new buffered channel with a specified buffer size.
    pub fn new<T: Send + 'static>(
        buffer_size: usize,
        tx: impl Fn(T) + Send + Sync + 'static,
    ) -> mpsc::Sender<T> {
        let (producer_sender, producer_receiver) = mpsc::channel::<T>();

        thread::spawn(move || {
            let mut buffer = Vec::with_capacity(buffer_size);
            let mut buffering = true;
            for item in producer_receiver {
                if buffering {
                    buffer.push(item);
                    if buffer.len() >= buffer_size {
                        for buffered_item in buffer.drain(..) {
                            tx(buffered_item);
                        }
                        buffering = false
                    }
                } else {
                    tx(item)
                }
            }
        });

        producer_sender
    }
}
