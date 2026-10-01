use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Identitas unik untuk setiap simpul atau pengguna dalam kolaborasi CRDT
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ClientId(pub String);

impl From<&str> for ClientId {
    fn from(s: &str) -> Self {
        ClientId(s.to_string())
    }
}

impl From<String> for ClientId {
    fn from(s: String) -> Self {
        ClientId(s)
    }
}

/// Stempel waktu logis (Lamport Timestamp) untuk penentuan urutan kausal
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LogicalClock {
    pub counter: u64,
}

impl LogicalClock {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn tick(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }

    pub fn update(&mut self, received: u64) {
        self.counter = self.counter.max(received) + 1;
    }
}

/// Last-Write-Wins Register (LWW-Register)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwwRegister<T: Clone> {
    pub value: T,
    pub timestamp: u64,
    pub client_id: ClientId,
}

impl<T: Clone> LwwRegister<T> {
    pub fn new(initial: T, client_id: ClientId) -> Self {
        Self {
            value: initial,
            timestamp: 0,
            client_id,
        }
    }

    pub fn set(&mut self, new_val: T, timestamp: u64, client_id: ClientId) {
        if timestamp > self.timestamp || (timestamp == self.timestamp && client_id.0 > self.client_id.0) {
            self.value = new_val;
            self.timestamp = timestamp;
            self.client_id = client_id;
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}

/// Posisi pecahan (Fractional Index) untuk Text CRDT yang mendukung penyisipan tak hingga
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FractionalIndex {
    pub path: Vec<u32>,
    pub client_id: ClientId,
}

impl FractionalIndex {
    pub fn between(before: Option<&FractionalIndex>, after: Option<&FractionalIndex>, client_id: ClientId) -> Self {
        let before_path = before.map(|b| b.path.as_slice()).unwrap_or(&[]);
        let after_path = after.map(|a| a.path.as_slice()).unwrap_or(&[]);

        let mut path = Vec::new();
        let mut idx = 0;

        loop {
            let low = *before_path.get(idx).unwrap_or(&0);
            let high = *after_path.get(idx).unwrap_or(&u32::MAX);

            if high > low + 1 {
                let mid = low + (high - low) / 2;
                path.push(mid);
                break;
            } else if high == low + 1 {
                path.push(low);
                idx += 1;
            } else {
                path.push(low);
                idx += 1;
            }

            if idx > 100 {
                path.push(50);
                break;
            }
        }

        Self { path, client_id }
    }
}

/// Karakter CRDT dengan penanda unik dan status aktif/terhapus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrdtChar {
    pub id: FractionalIndex,
    pub character: char,
    pub is_deleted: bool,
    pub lamport: u64,
}

/// Struktur Dokumen Teks Bebas Konflik (Sequence CRDT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextCrdt {
    pub client_id: ClientId,
    pub clock: LogicalClock,
    pub characters: Vec<CrdtChar>,
}

impl TextCrdt {
    pub fn new(client_id: impl Into<ClientId>) -> Self {
        Self {
            client_id: client_id.into(),
            clock: LogicalClock::new(),
            characters: Vec::new(),
        }
    }

    pub fn insert(&mut self, pos: usize, c: char) -> CrdtChar {
        let visible_chars: Vec<&CrdtChar> = self.characters.iter().filter(|ch| !ch.is_deleted).collect();
        let before = if pos > 0 && pos <= visible_chars.len() {
            Some(&visible_chars[pos - 1].id)
        } else {
            None
        };
        let after = if pos < visible_chars.len() {
            Some(&visible_chars[pos].id)
        } else {
            None
        };

        let new_id = FractionalIndex::between(before, after, self.client_id.clone());
        let lamport = self.clock.tick();
        let crdt_char = CrdtChar {
            id: new_id.clone(),
            character: c,
            is_deleted: false,
            lamport,
        };

        let idx = match self.characters.binary_search_by(|probe| probe.id.cmp(&new_id)) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.characters.insert(idx, crdt_char.clone());
        crdt_char
    }

    pub fn insert_text(&mut self, mut pos: usize, text: &str) {
        for c in text.chars() {
            self.insert(pos, c);
            pos += 1;
        }
    }

    pub fn delete(&mut self, pos: usize) -> Option<CrdtChar> {
        let mut visible_count = 0;
        let mut target_idx = None;

        for (i, ch) in self.characters.iter().enumerate() {
            if !ch.is_deleted {
                if visible_count == pos {
                    target_idx = Some(i);
                    break;
                }
                visible_count += 1;
            }
        }

        if let Some(idx) = target_idx {
            self.clock.tick();
            self.characters[idx].is_deleted = true;
            Some(self.characters[idx].clone())
        } else {
            None
        }
    }

    pub fn delete_range(&mut self, pos: usize, len: usize) {
        for _ in 0..len {
            self.delete(pos);
        }
    }

    pub fn integrate(&mut self, op: CrdtChar) {
        self.clock.update(op.lamport);
        match self.characters.binary_search_by(|probe| probe.id.cmp(&op.id)) {
            Ok(idx) => {
                if op.is_deleted {
                    self.characters[idx].is_deleted = true;
                }
            }
            Err(idx) => {
                self.characters.insert(idx, op);
            }
        }
    }

    pub fn read(&self) -> String {
        self.characters
            .iter()
            .filter(|ch| !ch.is_deleted)
            .map(|ch| ch.character)
            .collect()
    }

    pub fn read_text(&self) -> String {
        self.read()
    }
}

/// Pengguna dalam Sesi Kolaboratif
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollabUser {
    pub client_id: ClientId,
    pub name: String,
    pub color_hex: String,
    pub cursor_pos: usize,
    pub last_active_utc: String,
}

// ================= TRACK CHANGES & THREADED COMMENTS (ENTERPRISE) =================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeType {
    Insertion,
    Deletion,
    FormatChange,
}

/// Rekaman Perubahan Dokumen (Track Changes / Redlining)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackChange {
    pub id: String,
    pub author: String,
    pub change_type: ChangeType,
    pub text: String,
    pub original_text: Option<String>,
    pub position: usize,
    pub timestamp: String,
    pub is_accepted: Option<bool>,
}

/// Komentar Berulir (Threaded Review Comments)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadedComment {
    pub id: String,
    pub author: String,
    pub element_target: String,
    pub content: String,
    pub timestamp: String,
    pub is_resolved: bool,
    pub replies: Vec<ThreadedComment>,
}

/// Sesi Peninjauan & Kolaborasi Terpadu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReviewSession {
    pub document_id: String,
    pub changes: Vec<TrackChange>,
    pub comments: Vec<ThreadedComment>,
}

impl DocumentReviewSession {
    pub fn new(doc_id: &str) -> Self {
        Self {
            document_id: doc_id.to_string(),
            changes: Vec::new(),
            comments: Vec::new(),
        }
    }

    pub fn add_change(&mut self, author: &str, change_type: ChangeType, text: &str, position: usize) -> String {
        let id = format!("CHG-{}", self.changes.len() + 1);
        let now = chrono::Utc::now().to_rfc3339();
        self.changes.push(TrackChange {
            id: id.clone(),
            author: author.to_string(),
            change_type,
            text: text.to_string(),
            original_text: None,
            position,
            timestamp: now,
            is_accepted: None,
        });
        id
    }

    pub fn accept_change(&mut self, change_id: &str) -> bool {
        if let Some(chg) = self.changes.iter_mut().find(|c| c.id == change_id) {
            chg.is_accepted = Some(true);
            true
        } else {
            false
        }
    }

    pub fn reject_change(&mut self, change_id: &str) -> bool {
        if let Some(chg) = self.changes.iter_mut().find(|c| c.id == change_id) {
            chg.is_accepted = Some(false);
            true
        } else {
            false
        }
    }

    pub fn accept_all_changes(&mut self) {
        for c in &mut self.changes {
            if c.is_accepted.is_none() {
                c.is_accepted = Some(true);
            }
        }
    }

    pub fn add_comment(&mut self, author: &str, target: &str, content: &str) -> String {
        let id = format!("CMT-{}", self.comments.len() + 1);
        let now = chrono::Utc::now().to_rfc3339();
        self.comments.push(ThreadedComment {
            id: id.clone(),
            author: author.to_string(),
            element_target: target.to_string(),
            content: content.to_string(),
            timestamp: now,
            is_resolved: false,
            replies: Vec::new(),
        });
        id
    }

    pub fn reply_comment(&mut self, parent_id: &str, author: &str, content: &str) -> bool {
        let reply_id = format!("REP-{}", chrono::Utc::now().timestamp_millis());
        let now = chrono::Utc::now().to_rfc3339();

        if let Some(parent) = self.comments.iter_mut().find(|c| c.id == parent_id) {
            parent.replies.push(ThreadedComment {
                id: reply_id,
                author: author.to_string(),
                element_target: parent.element_target.clone(),
                content: content.to_string(),
                timestamp: now,
                is_resolved: false,
                replies: Vec::new(),
            });
            true
        } else {
            false
        }
    }

    pub fn resolve_comment(&mut self, comment_id: &str) -> bool {
        if let Some(c) = self.comments.iter_mut().find(|c| c.id == comment_id) {
            c.is_resolved = true;
            true
        } else {
            false
        }
    }
}

/// Sesi Kolaborasi Ruang Kerja
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollabSession {
    pub room_id: String,
    pub document_name: String,
    pub users: HashMap<ClientId, CollabUser>,
    pub crdt_engine: TextCrdt,
    pub review_session: DocumentReviewSession,
}

impl CollabSession {
    pub fn new(room_id: &str, doc_name: &str, host_client: impl Into<ClientId>, host_name: &str) -> Self {
        let client = host_client.into();
        let mut session = Self {
            room_id: room_id.to_string(),
            document_name: doc_name.to_string(),
            users: HashMap::new(),
            crdt_engine: TextCrdt::new(client.clone()),
            review_session: DocumentReviewSession::new(doc_name),
        };
        session.join(client, host_name, "#58a6ff");
        session
    }

    pub fn join(&mut self, client_id: impl Into<ClientId>, name: &str, color_hex: &str) {
        let cid = client_id.into();
        self.users.insert(
            cid.clone(),
            CollabUser {
                client_id: cid,
                name: name.to_string(),
                color_hex: color_hex.to_string(),
                cursor_pos: 0,
                last_active_utc: chrono::Utc::now().to_rfc3339(),
            },
        );
    }

    /// Render Antarmuka Editor Kolaboratif HTML5 Standalone
    pub fn render_html_editor(&self) -> String {
        let doc_text = self.crdt_engine.read();
        let user_list: String = self
            .users
            .values()
            .map(|u| format!("<span class='user-pill' style='border-left: 3px solid {};'>{}</span>", u.color_hex, u.name))
            .collect::<Vec<String>>()
            .join(" ");

        format!(
            r#"<!DOCTYPE html><html><head><meta charset='utf-8'><title>Kolaborasi: {}</title>
<style>
    body {{ font-family: 'Segoe UI', Tahoma, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 24px; }}
    .header {{ display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #334155; padding-bottom: 12px; margin-bottom: 16px; }}
    .user-pill {{ background: #1e293b; padding: 4px 10px; border-radius: 4px; font-size: 8.5pt; margin-right: 6px; }}
    textarea {{ width: 100%; height: 500px; background: #1e293b; color: #f8fafc; border: 1px solid #334155; border-radius: 8px; padding: 16px; font-family: 'Consolas', monospace; font-size: 11pt; box-sizing: border-box; }}
</style></head><body>
    <div class='header'>
        <div><h2>📝 Ruang Kolaborasi: {}</h2><div>Peserta: {}</div></div>
        <div style='color:#34d399; font-weight: bold;'>● CRDT LWW Aktif</div>
    </div>
    <textarea>{}</textarea>
</body></html>"#,
            self.document_name, self.room_id, user_list, doc_text
        )
    }
}
