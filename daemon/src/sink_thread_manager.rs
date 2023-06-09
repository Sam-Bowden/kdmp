use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct SinkThreadManager {
    pub current_sink: Arc<Mutex<Option<Sink>>>,
    pub transmitter: Option<Sender<()>>,
}

impl SinkThreadManager {
    pub fn new() -> SinkThreadManager {
        Self {
            current_sink: Arc::new(Mutex::new(None)),
            transmitter: None,
        }
    }

    pub fn play_tracks(&mut self, tracks: Vec<PathBuf>) {
        //If a communication channel to a sink thread still exists, attempt to tell the sink thread
        //to finish
        if let Some(t) = &self.transmitter {
            t.send(()).unwrap_or_default();
        }

        //Replace old communication channel with one for the new sink thread
        let receiver = match mpsc::channel() {
            (tx, rx) => {
                self.transmitter = Some(tx);
                rx
            }
        };

        //Create a reference to the current sink for the new sink thread
        let current_sink_ref = Arc::clone(&self.current_sink);

        thread::spawn(move || {
            //Setup sink and play with requested track
            let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();

            {
                let new_sink = Sink::try_new(&output_stream_handle).unwrap();
                for track in tracks {
                    let track_file = File::open(&track).unwrap();
                    let source = Decoder::new(BufReader::new(track_file)).unwrap();
                    new_sink.append(source);
                }
                new_sink.play();
                *current_sink_ref.lock().unwrap() = Some(new_sink);
            }

            //Spin wait for conditions in which the sink thread should end
            loop {
                thread::sleep(Duration::from_secs(1));

                //End thread when sink is empty
                if let Some(s) = &*current_sink_ref.lock().unwrap() {
                    if s.empty() {
                        break;
                    }
                }
                //End thread when sink has been replaced
                if let Ok(()) = receiver.try_recv() {
                    break;
                }
            }
        });
    }
}
