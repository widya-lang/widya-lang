pub mod braille_transcriber;
pub mod haptic_feedback;
pub mod liblouis_bridge;
pub mod spatial_screen_reader;

pub use braille_transcriber::{BrailleGrade, BrailleTranscriber};
pub use haptic_feedback::{HapticController, HapticImpulse, HapticPattern};
pub use liblouis_bridge::{BrailleDisplayDriver, BrailleDisplayInfo};
pub use spatial_screen_reader::{SpatialAudioCue, SpatialScreenReader};
