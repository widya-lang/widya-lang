use widya::learn::TutorialEngine;
use widya::embedded::{EmbeddedFirmwareGenerator, TargetArch};

#[test]
fn test_tutorial_engine_curriculum() {
    let curriculum = TutorialEngine::get_curriculum();
    assert_eq!(curriculum.len(), 6, "Harus memiliki 6 bab kurikulum interaktif");
    
    for ch in curriculum {
        assert!(!ch.title.is_empty());
        assert!(!ch.concept.is_empty());
        assert!(!ch.sample_code.is_empty());
    }
}

#[test]
fn test_embedded_firmware_generator() {
    let temp_dir = std::env::temp_dir().join("widya_embedded_test");
    let arch = TargetArch::ArmCortexM4F;
    let res = EmbeddedFirmwareGenerator::emit_firmware_package(&temp_dir, "firmware_test", &arch);
    assert!(res.is_ok());

    let ld_path = res.unwrap();
    assert!(ld_path.exists());
    let ld_content = std::fs::read_to_string(&ld_path).unwrap();
    assert!(ld_content.contains("MEMORY"));
    assert!(ld_content.contains("FLASH"));
    assert!(ld_content.contains("RAM"));
}
