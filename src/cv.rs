extern crate gstreamer as gst;
//use gst::prelude::*;





use gst::prelude::*; // Import necessary modules

pub fn displayVideo() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Build the pipeline
    let pipeline_str = "filesrc location=./Hack_Project3.0.mp4 ! decodebin ! autovideosink";
    let pipeline = gst::gst_parse_launch(pipeline_str).unwrap();

    // Start playing
    pipeline.set_state(gst::State::Playing).unwrap();

    // Wait until error or EOS
    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => break,
            gst::MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string().unwrap_or_else(|| "".to_string())), // Use path_string() instead of get_path()
                    err.error(),
                    err.debug(),
                );
                break;
            },
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline.set_state(gst::State::Null).unwrap();
}





/* 
pub fn displayVideo(){
    // Initialize GStreamer
    gst::init().unwrap();

    // Build the pipeline
    let pipeline_str = "filesrc location=./Hack_Project3.0.mp4 ! decodebin ! autovideosink";
    let pipeline = gst::parse_launch(&pipeline_str).unwrap();

    // Start playing
    pipeline.set_state(gst::State::Playing).unwrap();

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => break,
            gst::MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.get_path().as_str()),
                    err.error(),
                    err.debug(),
                );
                break;
            },
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline.set_state(gst::State::Null).unwrap();
}
 */