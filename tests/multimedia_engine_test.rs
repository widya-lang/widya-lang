//! Integration Test Suite: Multimedia Photo & Video Editing Engine

use widya::multimedia::image::{ImageBuffer, RgbaPixel, BlendMode};
use widya::multimedia::video::{
    VideoTimeline, VideoTrack, VideoClip, AnimationTrack, Keyframe, EasingType, VideoTransition, TransitionType
};

#[test]
fn test_photo_color_grading_and_transforms() {
    let mut img = ImageBuffer::new(4, 4, RgbaPixel::rgb(100, 150, 200));

    // 1. Uji Brightness
    img.adjust_brightness(0.1);
    let p = img.get_pixel(0, 0).unwrap();
    assert!(p.r > 100 && p.g > 150 && p.b > 200, "Brightness harus menaikkan nilai RGB");

    // 2. Uji Grayscale
    img.to_grayscale();
    let p_gray = img.get_pixel(0, 0).unwrap();
    assert_eq!(p_gray.r, p_gray.g, "Kanal R dan G harus sama pada Grayscale");
    assert_eq!(p_gray.g, p_gray.b, "Kanal G dan B harus sama pada Grayscale");

    // 3. Uji Invert
    let mut inv_img = ImageBuffer::new(2, 2, RgbaPixel::BLACK);
    inv_img.invert();
    assert_eq!(inv_img.get_pixel(0, 0).unwrap(), RgbaPixel::WHITE);

    // 4. Uji Resizing Bilinear
    let orig = ImageBuffer::new(2, 2, RgbaPixel::rgb(255, 0, 0));
    let resized = orig.resize_bilinear(4, 4);
    assert_eq!(resized.width, 4);
    assert_eq!(resized.height, 4);
    assert_eq!(resized.get_pixel(2, 2).unwrap().r, 255);
}

#[test]
fn test_photo_convolution_kernels_and_blending() {
    let mut canvas = ImageBuffer::new(5, 5, RgbaPixel::rgb(128, 128, 128));

    // Set high contrast center pixel
    canvas.set_pixel(2, 2, RgbaPixel::WHITE);

    // 1. Uji Gaussian Blur
    let blurred = canvas.gaussian_blur();
    let center = blurred.get_pixel(2, 2).unwrap();
    assert!(center.r < 255, "Blur harus melunakkan piksel tengah berintensitas tinggi");

    // 2. Uji Sharpen
    let sharpened = canvas.sharpen();
    assert_eq!(sharpened.width, 5);

    // 3. Uji Sobel Edge Detection
    let edges = canvas.edge_detect_sobel();
    assert_eq!(edges.width, 5);

    // 4. Uji Layer Blending (Multiply)
    let mut base = ImageBuffer::new(2, 2, RgbaPixel::rgb(200, 200, 200));
    let overlay = ImageBuffer::new(2, 2, RgbaPixel::rgb(100, 100, 100));
    base.blend_layer(&overlay, 0, 0, BlendMode::Multiply, 1.0);

    let blended = base.get_pixel(0, 0).unwrap();
    // 200 * 100 / 255 = 78
    assert!(blended.r <= 80 && blended.r >= 76, "Multiply blending harus menggelapkan warna");
}

#[test]
fn test_video_keyframe_animation() {
    let mut track = AnimationTrack::new();
    track.add_keyframe(0, 0.0, EasingType::Linear);
    track.add_keyframe(1000, 100.0, EasingType::Linear);

    assert_eq!(track.evaluate(0, 0.0), 0.0);
    assert_eq!(track.evaluate(500, 0.0), 50.0);
    assert_eq!(track.evaluate(1000, 0.0), 100.0);
}

#[test]
fn test_video_timeline_nle_compositor() {
    let mut timeline = VideoTimeline::new(64, 64, 30.0, 3000);

    // Track 1: Klip Merah (0 s/d 2000 ms)
    let frame_red = ImageBuffer::new(64, 64, RgbaPixel::rgb(255, 0, 0));
    let clip1 = VideoClip::new("CLIP-RED", 0, 2000, vec![frame_red], 30.0);

    let mut track1 = VideoTrack::new("TRACK-VIDEO-1");
    track1.add_clip(clip1);
    timeline.add_track(track1);

    // Track 2: Klip Biru Overlay (1000 s/d 3000 ms)
    let frame_blue = ImageBuffer::new(32, 32, RgbaPixel::rgb(0, 0, 255));
    let mut clip2 = VideoClip::new("CLIP-BLUE", 1000, 2000, vec![frame_blue], 30.0);
    clip2.pos_x_track.add_keyframe(0, 10.0, EasingType::Linear);
    clip2.pos_y_track.add_keyframe(0, 10.0, EasingType::Linear);

    let mut track2 = VideoTrack::new("TRACK-VIDEO-2");
    track2.add_clip(clip2);
    timeline.add_track(track2);

    // Transisi Fade to Black di akhir (2500 s/d 3000 ms)
    timeline.add_transition(VideoTransition {
        transition_type: TransitionType::FadeToBlack,
        start_time_ms: 2500,
        duration_ms: 500,
    });

    // 1. Render Frame pada t = 500 ms (Hanya Klip Merah)
    let frame_500 = timeline.render_frame(500);
    let p_500 = frame_500.get_pixel(0, 0).unwrap();
    assert_eq!(p_500.r, 255);
    assert_eq!(p_500.b, 0);

    // 2. Render Frame pada t = 1500 ms (Klip Merah + Overlay Biru pada koordinat 10, 10)
    let frame_1500 = timeline.render_frame(1500);
    let p_overlay = frame_1500.get_pixel(10, 10).unwrap();
    assert_eq!(p_overlay.b, 255, "Piksel overlay harus berwarna biru");

    // 3. Render Frame pada t = 2750 ms (Sedang proses Fade To Black)
    let frame_fade = timeline.render_frame(2750);
    assert!(frame_fade.width == 64 && frame_fade.height == 64);
}
