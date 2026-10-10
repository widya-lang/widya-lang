//! Test Integrasi 100% Berdaulat: WidyaStream & WidyaOBS Parity
//! Zero External Dependencies (Tanpa OBS Studio, FFmpeg CLI, LiveKit, atau Agora).

use widya::web::broadcast::{
    BroadcastStudioEngine, BroadcastSource, BroadcastScene, StreamEgressConfig,
    SourceType, SceneLayoutType, TransitionType, SmpteTimecode,
};

#[test]
fn test_broadcast_studio_scene_management_and_transitions() {
    let studio = BroadcastStudioEngine::new("studio-01", "Studio Utama WidyaTV");

    // 1. Tambah Sumber Input Kamera & Layar
    let cam1 = BroadcastSource::new("cam-1", "Kamera Utama Presenter (Webcam)", SourceType::Webcam);
    let cam2 = BroadcastSource::new("cam-2", "Kamera HP Wireless (Mevo Go Equivalent)", SourceType::RemoteStream);
    let screen = BroadcastSource::new("screen-1", "Tangkapan Layar Monitor", SourceType::ScreenShare);

    studio.add_source(cam1);
    studio.add_source(cam2);
    studio.add_source(screen);

    assert_eq!(studio.sources.lock().unwrap().len(), 3);

    // 2. Buat Adegan (Scenes)
    let mut scene_talk = BroadcastScene::new("scene-talk", "Presenter Full", SceneLayoutType::Solo);
    scene_talk.setup_default_layout(&["cam-1".to_string()]);

    let mut scene_pip = BroadcastScene::new("scene-pip", "Presentasi + PiP Kamera", SceneLayoutType::PictureInPicture);
    scene_pip.setup_default_layout(&["screen-1".to_string(), "cam-2".to_string()]);

    studio.add_scene(scene_talk);
    studio.add_scene(scene_pip);

    assert_eq!(studio.scenes.lock().unwrap().len(), 2);

    // 3. Uji Studio Mode: Set Preview vs Switch Program Live
    assert!(studio.select_preview_scene("scene-pip").is_ok());
    assert_eq!(*studio.preview_scene_id.lock().unwrap(), "scene-pip");

    // Cut to Program
    let res_cut = studio.transition_to_program(TransitionType::Cut, 0);
    assert!(res_cut.is_ok());
    assert_eq!(*studio.program_scene_id.lock().unwrap(), "scene-pip");

    // Preview scene-talk lalu Fade transition
    assert!(studio.select_preview_scene("scene-talk").is_ok());
    let res_fade = studio.transition_to_program(TransitionType::CrossFade, 300);
    assert!(res_fade.is_ok());
    assert_eq!(*studio.program_scene_id.lock().unwrap(), "scene-talk");

    // 4. Uji Tally Light Status (Mevo Go & Studio Tally)
    let tally = studio.get_tally_status();
    assert_eq!(tally.get("cam-1"), Some(&"PROGRAM_RED_ON_AIR".to_string()));
}

#[test]
fn test_multi_destination_streaming_egress_and_telemetry() {
    let studio = BroadcastStudioEngine::new("studio-02", "Studio Siaran Nasional");

    let mut config = StreamEgressConfig::default();
    config.rtmp_relay_url = Some("rtmp://a.rtmp.youtube.com/live2/xxxx-xxxx".to_string());
    config.webrtc_whep_enabled = true;
    config.edge_webtransport_enabled = true;
    config.target_bitrate_kbps = 6000;
    config.target_fps = 60;

    let telem = studio.start_streaming(config);
    assert!(telem.is_live);
    assert_eq!(telem.active_viewers_edge, 1240);

    let current = studio.get_telemetry();
    assert!(current.is_live);

    studio.stop_streaming();
    assert!(!studio.get_telemetry().is_live);
}

#[test]
fn test_smpte_timecode_and_audio_mixer() {
    // SMPTE Timecode precision 60 FPS
    let tc = SmpteTimecode::new(1, 30, 0, 0, 60);
    assert_eq!(tc.to_string_repr(), "01:30:00:00");
    let total_frames = tc.to_total_frames();
    assert_eq!(total_frames, (3600 + 1800) * 60);

    let tc_added = tc.add_frames(120); // Tambah 2 detik (120 frames di 60fps)
    assert_eq!(tc_added.to_string_repr(), "01:30:02:00");

    // Audio Mixer Strip
    let studio = BroadcastStudioEngine::new("studio-03", "Audio Podcast");
    let mic = BroadcastSource::new("mic-01", "Shure SM7B Host", SourceType::Webcam);
    studio.add_source(mic);

    let mixer = studio.mixer.lock().unwrap();
    assert!(mixer.channels.contains_key("ch_mic-01"));
    let strip = mixer.channels.get("ch_mic-01").unwrap();
    assert!(strip.noise_gate_active);
    assert_eq!(strip.fader_db, 0.0);
}
