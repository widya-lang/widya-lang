use serde::{Deserialize, Serialize};

/// ============================================================================
/// MANAJEMEN PROYEK AGILE, SCRUM & KANBAN (FIBONACCI & BURNDOWN)
/// Sprints, User Stories, Story Points, Velocity, dan WIP Limits
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrioritasMoscow {
    MustHave,   // Wajib ada pada rilis ini
    ShouldHave, // Sangat penting tetapi ada alternatif sementara
    CouldHave,  // Bagus jika ada sumber daya luang
    WontHave,   // Ditunda untuk rilis berikutnya
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusKanban {
    Backlog,
    ToDo,
    InProgress,
    InCodeReviewQa,
    Done,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserStory {
    pub id_story: String,           // e.g. "US-101"
    pub judul: String,
    pub sebagai_peran: String,      // "As a ..."
    pub saya_ingin: String,         // "I want to ..."
    pub sehingga_tujuan: String,    // "So that ..."
    pub story_points: u32,          // Fibonacci: 1, 2, 3, 5, 8, 13, 21
    pub prioritas: PrioritasMoscow,
    pub status: StatusKanban,
    pub assignee: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintScrum {
    pub nomor_sprint: u32,
    pub nama_sprint: String,        // e.g. "Sprint 14: Integrasi SIPLah & ARKAS"
    pub tanggal_mulai: String,
    pub tanggal_selesai: String,
    pub target_komitmen_points: u32,
    pub daftar_stories: Vec<UserStory>,
    pub points_terselesaikan: u32,
    pub status_sprint_aktif: bool,
}

impl SprintScrum {
    pub fn new(nomor: u32, nama: &str, mulai: &str, selesai: &str) -> Self {
        Self {
            nomor_sprint: nomor,
            nama_sprint: nama.to_string(),
            tanggal_mulai: mulai.to_string(),
            tanggal_selesai: selesai.to_string(),
            target_komitmen_points: 0,
            daftar_stories: Vec::new(),
            points_terselesaikan: 0,
            status_sprint_aktif: true,
        }
    }

    pub fn tambah_story(&mut self, story: UserStory) {
        self.target_komitmen_points += story.story_points;
        self.daftar_stories.push(story);
    }

    pub fn selesaikan_story(&mut self, id_story: &str) -> Result<(), String> {
        let story = self.daftar_stories.iter_mut().find(|s| s.id_story == id_story)
            .ok_or_else(|| format!("User Story '{}' tidak ditemukan di Sprint!", id_story))?;

        story.status = StatusKanban::Done;
        self.points_terselesaikan += story.story_points;
        Ok(())
    }

    /// Menghitung sisa poin untuk Burndown Chart
    pub fn sisa_points_burndown(&self) -> u32 {
        if self.target_komitmen_points >= self.points_terselesaikan {
            self.target_komitmen_points - self.points_terselesaikan
        } else {
            0
        }
    }
}
