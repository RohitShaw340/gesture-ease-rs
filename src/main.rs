use gesture_ease::Models;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{
    CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution,
};
use nokhwa::Camera;
use std::io::Cursor;
use std::os::unix::net::UnixListener;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let socket_path = "/tmp/gesurease.sock";
    let num_processes = 1;

    if std::fs::metadata(socket_path).is_ok() {
        println!("Socket is already present. Deleting...");
        std::fs::remove_file(socket_path).unwrap();
    }

    let listener = UnixListener::bind(socket_path).unwrap();
    let mut process_map = Models::new(num_processes, listener);

    let index = CameraIndex::Index(0);
    let h = 720;
    let w = 1280;
    let resolution = Resolution::new(w, h);
    let frame_format = FrameFormat::MJPEG;
    let camera_format = CameraFormat::new(resolution, frame_format, 30);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(camera_format));

    let mut camera = Camera::new(index, requested).unwrap();
    camera.open_stream().unwrap();

    let mut img = Cursor::new(vec![]);

    process_map.wait_for_connection();

    loop {
        let start = Instant::now();
        let frame = camera.frame().unwrap();
        let img_bug = frame.decode_image::<RgbFormat>().unwrap();
        img_bug
            .write_to(&mut img, image::ImageFormat::Jpeg)
            .unwrap();

        let buffer: Arc<[u8]> = img.get_ref().to_owned().into();

        process_map.hpe().unwrap().send(buffer.clone()).unwrap();
        let _res = process_map.hpe().unwrap().recv().unwrap();

        img.set_position(0);
        let duration = Instant::now().duration_since(start).as_millis();
        println!("duration in ms: {}", duration);
    }
}
