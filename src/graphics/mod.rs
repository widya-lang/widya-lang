//! Submodul Grafis & Kanvas Murni (Zero External Dependency)
//! Mengimplementasikan 2D Software Rasterizer, Framebuffer RGBA, Font Engine,
//! Three-Tree Reactive UI Engine, Gesture/Animation Engine, Material 3 Widgets,
//! Reactive State Management, dan Page Router Deep Linking.

pub mod pure_canvas;
pub mod pure_font;
pub mod pure_engine;
pub mod pure_events;
pub mod pure_widgets;
pub mod pure_state;
pub mod pure_router;
pub mod pure_apk;
pub mod pure_ipa;

pub use pure_canvas::{FramebufferMurni, PersegiKotak, Titik2D, VektorPath, Warna};
pub use pure_font::{FontAtlasMurni, GayaTipografi, MetrikTeks, PerataanTeks};
pub use pure_engine::{BatasanKotak, JenisWidget, RenderKotakMurni, WidgetMurni};
pub use pure_events::{
    GesturTerdeteksi, HitTestDispatcher, JenisKurva, JenisSentuhan, PengontrolAnimasi,
    PeristiwaSentuh, StatusAnimasi, Tween,
};
pub use pure_widgets::WidyaWidgets;
pub use pure_state::{PemberitahuNilai, Sinyal};
pub use pure_router::{RuteHalaman, WidyaNavigator};
pub use pure_apk::*;
pub use pure_ipa::*;
