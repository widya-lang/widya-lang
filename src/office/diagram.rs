use crate::office::converter::clean_markdown_inline;
use crate::office::pdf::{wrap_text, PdfDocument, PdfPage};
use std::collections::{HashMap, HashSet};

/// Arah Aliran Diagram
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramDirection {
    TopToBottom,
    LeftToRight,
}

/// Bentuk Node Diagram
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeShape {
    Rectangle,
    Rounded,
    Diamond,
}

/// Simpul / Node dalam Diagram
#[derive(Debug, Clone)]
pub struct DiagramNode {
    pub id: String,
    pub label: String,
    pub shape: NodeShape,
    pub subgraph_id: Option<String>,
}

/// Garis Penghubung / Relasi antar Node
#[derive(Debug, Clone)]
pub struct DiagramEdge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub has_arrow: bool,
}

/// Grup / Subgraph Diagram
#[derive(Debug, Clone)]
pub struct DiagramSubgraph {
    pub id: String,
    pub title: String,
    pub node_ids: Vec<String>,
}

/// Struktur Data Diagram Arsitektur Berdaulat
#[derive(Debug, Clone)]
pub struct WidyaDiagram {
    pub direction: DiagramDirection,
    pub subgraphs: Vec<DiagramSubgraph>,
    pub nodes: Vec<DiagramNode>,
    pub edges: Vec<DiagramEdge>,
}

impl WidyaDiagram {
    /// Parsing Sintaks Diagram Mermaid Dasar (flowchart TD/LR, subgraph, nodes, edges)
    pub fn parse(source: &str) -> Self {
        let mut direction = DiagramDirection::TopToBottom;
        let mut subgraphs = Vec::new();
        let mut nodes_map: HashMap<String, DiagramNode> = HashMap::new();
        let mut nodes_order: Vec<String> = Vec::new();
        let mut edges = Vec::new();

        let mut current_subgraph: Option<String> = None;

        for raw_line in source.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            // 1. Deklarasi Tipe & Arah Diagram
            if line.starts_with("flowchart") || line.starts_with("graph") {
                if line.contains("LR") {
                    direction = DiagramDirection::LeftToRight;
                } else {
                    direction = DiagramDirection::TopToBottom;
                }
                continue;
            }

            // 2. Subgraph (subgraph ID["Title"] atau subgraph Title)
            if line.starts_with("subgraph") {
                let rest = line.trim_start_matches("subgraph").trim();
                let (id, title) = Self::parse_subgraph_header(rest);
                let clean_title = clean_markdown_inline(&title);
                current_subgraph = Some(id.clone());
                subgraphs.push(DiagramSubgraph {
                    id,
                    title: clean_title,
                    node_ids: Vec::new(),
                });
                continue;
            }

            // 3. Akhir Subgraph (end)
            if line == "end" {
                current_subgraph = None;
                continue;
            }

            // 4. Garis Hubung / Edge (--> atau --- atau -.->)
            if line.contains("-->") || line.contains("---") || line.contains("-.->") || line.contains("==>") {
                let arrow_pattern = if line.contains("-->") {
                    "-->"
                } else if line.contains("-.->") {
                    "-.->"
                } else if line.contains("==>") {
                    "==>"
                } else {
                    "---"
                };

                let parts: Vec<&str> = line.split(arrow_pattern).collect();
                if parts.len() >= 2 {
                    let from_str = parts[0].trim();
                    let to_str = parts[1].trim();

                    let from_node = Self::extract_or_create_node(from_str, &current_subgraph);
                    let to_node = Self::extract_or_create_node(to_str, &current_subgraph);

                    // Daftarkan node jika belum ada
                    Self::register_node(&mut nodes_map, &mut nodes_order, &mut subgraphs, from_node.clone());
                    Self::register_node(&mut nodes_map, &mut nodes_order, &mut subgraphs, to_node.clone());

                    edges.push(DiagramEdge {
                        from: from_node.id,
                        to: to_node.id,
                        label: None,
                        has_arrow: arrow_pattern != "---",
                    });
                    continue;
                }
            }

            // 5. Node Mandiri: ID["Label"] atau ID[Label]
            if let Some(node) = Self::parse_single_node(line, &current_subgraph) {
                Self::register_node(&mut nodes_map, &mut nodes_order, &mut subgraphs, node);
            }
        }

        let nodes = nodes_order.into_iter().filter_map(|id| nodes_map.remove(&id)).collect();

        Self {
            direction,
            subgraphs,
            nodes,
            edges,
        }
    }

    fn parse_subgraph_header(s: &str) -> (String, String) {
        if let Some(open_bracket) = s.find('[') {
            let id = s[..open_bracket].trim().to_string();
            let mut title = s[open_bracket + 1..].trim_end_matches(']').trim();
            if title.starts_with('"') && title.ends_with('"') && title.len() >= 2 {
                title = &title[1..title.len() - 1];
            }
            (id, title.to_string())
        } else {
            let clean = s.trim_matches('"').trim();
            (clean.to_string(), clean.to_string())
        }
    }

    fn extract_or_create_node(s: &str, current_subgraph: &Option<String>) -> DiagramNode {
        if let Some(node) = Self::parse_single_node(s, current_subgraph) {
            node
        } else {
            let id = s.trim().to_string();
            DiagramNode {
                id: id.clone(),
                label: id,
                shape: NodeShape::Rectangle,
                subgraph_id: current_subgraph.clone(),
            }
        }
    }

    fn parse_single_node(s: &str, current_subgraph: &Option<String>) -> Option<DiagramNode> {
        let trimmed = s.trim();
        // ID["Label"]
        if let Some(b_start) = trimmed.find('[') {
            if trimmed.ends_with(']') {
                let id = trimmed[..b_start].trim().to_string();
                let mut label = trimmed[b_start + 1..trimmed.len() - 1].trim();
                if label.starts_with('"') && label.ends_with('"') && label.len() >= 2 {
                    label = &label[1..label.len() - 1];
                }
                return Some(DiagramNode {
                    id,
                    label: clean_markdown_inline(label),
                    shape: NodeShape::Rectangle,
                    subgraph_id: current_subgraph.clone(),
                });
            }
        }
        // ID("Label")
        if let Some(b_start) = trimmed.find('(') {
            if trimmed.ends_with(')') {
                let id = trimmed[..b_start].trim().to_string();
                let mut label = trimmed[b_start + 1..trimmed.len() - 1].trim();
                if label.starts_with('"') && label.ends_with('"') && label.len() >= 2 {
                    label = &label[1..label.len() - 1];
                }
                return Some(DiagramNode {
                    id,
                    label: clean_markdown_inline(label),
                    shape: NodeShape::Rounded,
                    subgraph_id: current_subgraph.clone(),
                });
            }
        }
        // ID{"Label"}
        if let Some(b_start) = trimmed.find('{') {
            if trimmed.ends_with('}') {
                let id = trimmed[..b_start].trim().to_string();
                let mut label = trimmed[b_start + 1..trimmed.len() - 1].trim();
                if label.starts_with('"') && label.ends_with('"') && label.len() >= 2 {
                    label = &label[1..label.len() - 1];
                }
                return Some(DiagramNode {
                    id,
                    label: clean_markdown_inline(label),
                    shape: NodeShape::Diamond,
                    subgraph_id: current_subgraph.clone(),
                });
            }
        }
        None
    }

    fn register_node(
        map: &mut HashMap<String, DiagramNode>,
        order: &mut Vec<String>,
        subgraphs: &mut [DiagramSubgraph],
        node: DiagramNode,
    ) {
        let id = node.id.clone();
        if let Some(sg_id) = &node.subgraph_id {
            if let Some(sg) = subgraphs.iter_mut().find(|s| &s.id == sg_id) {
                if !sg.node_ids.contains(&id) {
                    sg.node_ids.push(id.clone());
                }
            }
        }
        if !map.contains_key(&id) {
            order.push(id.clone());
            map.insert(id, node);
        }
    }
}

/// Jenis Panah / Relasi dalam Diagram Urutan (Sequence Diagram)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceArrowType {
    SolidArrow,   // ->>
    DashedArrow,  // -->>
    SolidLine,    // ->
    DashedLine,   // -->
}

/// Pesan Interaksi antar Partisipan
#[derive(Debug, Clone)]
pub struct SequenceMessage {
    pub from: String,
    pub to: String,
    pub arrow: SequenceArrowType,
    pub text: String,
}

/// Partisipan / Entitas dalam Sequence Diagram
#[derive(Debug, Clone)]
pub struct SequenceParticipant {
    pub id: String,
    pub label: String,
}

/// Struktur Data Diagram Urutan (Mermaid Sequence Diagram)
#[derive(Debug, Clone)]
pub struct WidyaSequenceDiagram {
    pub title: Option<String>,
    pub participants: Vec<SequenceParticipant>,
    pub messages: Vec<SequenceMessage>,
}

impl WidyaSequenceDiagram {
    pub fn parse(source: &str) -> Option<Self> {
        let mut is_sequence = false;
        let mut title = None;
        let mut participants: Vec<SequenceParticipant> = Vec::new();
        let mut messages: Vec<SequenceMessage> = Vec::new();

        for raw_line in source.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            if line.starts_with("sequenceDiagram") {
                is_sequence = true;
                continue;
            }

            if !is_sequence {
                continue;
            }

            if line.starts_with("title:") || line.starts_with("title ") {
                let t = line.trim_start_matches("title:").trim_start_matches("title").trim();
                title = Some(clean_markdown_inline(t));
                continue;
            }

            if line.starts_with("autonumber") {
                continue;
            }

            // Deklarasi Partisipan / Actor: participant A as Alice atau participant A
            if line.starts_with("participant") || line.starts_with("actor") {
                let rest = if line.starts_with("participant") {
                    line.trim_start_matches("participant").trim()
                } else {
                    line.trim_start_matches("actor").trim()
                };

                let (id, label) = if let Some(as_pos) = rest.find(" as ") {
                    let id = rest[..as_pos].trim().to_string();
                    let lbl = rest[as_pos + 4..].trim().trim_matches('"').to_string();
                    (id, lbl)
                } else {
                    let clean = rest.trim_matches('"').to_string();
                    (clean.clone(), clean)
                };

                if !participants.iter().any(|p| p.id == id) {
                    participants.push(SequenceParticipant {
                        id,
                        label: clean_markdown_inline(&label),
                    });
                }
                continue;
            }

            // Pesan Interaksi: A->>B: text atau A-->>B: text atau A->B: text atau A-->B: text
            let arrow_info = if line.contains("-->>") {
                Some(("-->>", SequenceArrowType::DashedArrow))
            } else if line.contains("->>") {
                Some(("->>", SequenceArrowType::SolidArrow))
            } else if line.contains("-->") {
                Some(("-->", SequenceArrowType::DashedLine))
            } else if line.contains("->") {
                Some(("->", SequenceArrowType::SolidLine))
            } else {
                None
            };

            if let Some((arrow_str, arrow_type)) = arrow_info {
                let (conn_part, text_part) = if let Some(colon_pos) = line.find(':') {
                    (&line[..colon_pos], line[colon_pos + 1..].trim())
                } else {
                    (line, "")
                };

                let parts: Vec<&str> = conn_part.split(arrow_str).collect();
                if parts.len() >= 2 {
                    let from = parts[0].trim().to_string();
                    let to = parts[1].trim().to_string();

                    // Daftarkan partisipan otomatis jika belum ada
                    for part_id in [&from, &to] {
                        if !participants.iter().any(|p| p.id == **part_id) {
                            participants.push(SequenceParticipant {
                                id: (*part_id).clone(),
                                label: clean_markdown_inline(part_id),
                            });
                        }
                    }

                    messages.push(SequenceMessage {
                        from,
                        to,
                        arrow: arrow_type,
                        text: clean_markdown_inline(text_part),
                    });
                }
            }
        }

        if is_sequence {
            Some(Self {
                title,
                participants,
                messages,
            })
        } else {
            None
        }
    }
}

/// Transisi Status Mesin (State Machine Transition)
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
}

/// Struktur Data Diagram Mesin Status (Mermaid stateDiagram-v2)
#[derive(Debug, Clone)]
pub struct WidyaStateDiagram {
    pub title: Option<String>,
    pub states: Vec<String>,
    pub transitions: Vec<StateTransition>,
}

impl WidyaStateDiagram {
    pub fn parse(source: &str) -> Option<Self> {
        let mut is_state = false;
        let mut title = None;
        let mut states: Vec<String> = Vec::new();
        let mut transitions: Vec<StateTransition> = Vec::new();

        for raw_line in source.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            if line.starts_with("stateDiagram") {
                is_state = true;
                continue;
            }

            if !is_state {
                continue;
            }

            if line.starts_with("title:") || line.starts_with("title ") {
                let t = line.trim_start_matches("title:").trim_start_matches("title").trim();
                title = Some(clean_markdown_inline(t));
                continue;
            }

            // Transisi: A --> B atau A --> B: label
            if line.contains("-->") {
                let (conn_part, label_part) = if let Some(colon_pos) = line.find(':') {
                    (&line[..colon_pos], Some(line[colon_pos + 1..].trim()))
                } else {
                    (line, None)
                };

                let parts: Vec<&str> = conn_part.split("-->").collect();
                if parts.len() >= 2 {
                    let from = parts[0].trim().to_string();
                    let to = parts[1].trim().to_string();

                    for s in [&from, &to] {
                        if s != "[*]" && !states.contains(s) {
                            states.push(s.clone());
                        }
                    }

                    transitions.push(StateTransition {
                        from,
                        to,
                        label: label_part.map(|l| clean_markdown_inline(l)),
                    });
                }
            } else if line.starts_with("state ") {
                let rest = line.trim_start_matches("state").trim();
                let name = rest.split_whitespace().next().unwrap_or("").trim_matches('"');
                if !name.is_empty() && name != "[*]" && !states.contains(&name.to_string()) {
                    states.push(name.to_string());
                }
            }
        }

        if is_state {
            Some(Self {
                title,
                states,
                transitions,
            })
        } else {
            None
        }
    }
}

/// Anggota Kelas (Atribut / Field atau Metode / Fungsi)
#[derive(Debug, Clone)]
pub struct ClassMember {
    pub is_method: bool,
    pub text: String,
}

/// Entitas Kelas UML
#[derive(Debug, Clone)]
pub struct ClassEntity {
    pub name: String,
    pub members: Vec<ClassMember>,
}

/// Relasi Antar Kelas UML
#[derive(Debug, Clone)]
pub struct ClassRelation {
    pub from: String,
    pub to: String,
    pub rel_type: String, // "<|--", "*--", "o--", "-->", "--"
    pub label: Option<String>,
}

/// Struktur Data Diagram Kelas (Mermaid classDiagram)
#[derive(Debug, Clone)]
pub struct WidyaClassDiagram {
    pub title: Option<String>,
    pub classes: Vec<ClassEntity>,
    pub relations: Vec<ClassRelation>,
}

impl WidyaClassDiagram {
    pub fn parse(source: &str) -> Option<Self> {
        let mut is_class = false;
        let mut title = None;
        let mut classes_map: HashMap<String, ClassEntity> = HashMap::new();
        let mut classes_order: Vec<String> = Vec::new();
        let mut relations: Vec<ClassRelation> = Vec::new();

        let mut inside_class: Option<String> = None;

        for raw_line in source.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            if line.starts_with("classDiagram") {
                is_class = true;
                continue;
            }

            if !is_class {
                continue;
            }

            if line.starts_with("title:") || line.starts_with("title ") {
                let t = line.trim_start_matches("title:").trim_start_matches("title").trim();
                title = Some(clean_markdown_inline(t));
                continue;
            }

            // Blok penutup kelas
            if line == "}" {
                inside_class = None;
                continue;
            }

            // Di dalam blok kelas: field atau method
            if let Some(ref cls_name) = inside_class {
                let is_method = line.contains('(') && line.contains(')');
                if let Some(cls) = classes_map.get_mut(cls_name) {
                    cls.members.push(ClassMember {
                        is_method,
                        text: clean_markdown_inline(line),
                    });
                }
                continue;
            }

            // Definisi blok kelas: class Nama {
            if line.starts_with("class ") && line.ends_with('{') {
                let name = line.trim_start_matches("class")
                    .trim_end_matches('{')
                    .trim()
                    .to_string();
                if !classes_map.contains_key(&name) {
                    classes_order.push(name.clone());
                    classes_map.insert(name.clone(), ClassEntity {
                        name: name.clone(),
                        members: Vec::new(),
                    });
                }
                inside_class = Some(name);
                continue;
            }

            // Definisi single class: class Nama
            if line.starts_with("class ") {
                let name = line.trim_start_matches("class").trim().to_string();
                if !classes_map.contains_key(&name) {
                    classes_order.push(name.clone());
                    classes_map.insert(name.clone(), ClassEntity {
                        name,
                        members: Vec::new(),
                    });
                }
                continue;
            }

            // Relasi kelas: ClassA <|-- ClassB atau ClassA *-- ClassB dsb.
            let rel_patterns = ["<|--", "*--", "o--", "-->", "--", "--|>"];
            let mut found_rel = None;
            for &p in &rel_patterns {
                if line.contains(p) {
                    found_rel = Some(p);
                    break;
                }
            }

            if let Some(pat) = found_rel {
                let parts: Vec<&str> = line.split(pat).collect();
                if parts.len() >= 2 {
                    let from = parts[0].trim().to_string();
                    let to = parts[1].trim().to_string();

                    for c in [&from, &to] {
                        if !classes_map.contains_key(c) {
                            classes_order.push(c.clone());
                            classes_map.insert(c.clone(), ClassEntity {
                                name: c.clone(),
                                members: Vec::new(),
                            });
                        }
                    }

                    relations.push(ClassRelation {
                        from,
                        to,
                        rel_type: pat.to_string(),
                        label: None,
                    });
                }
                continue;
            }

            // Format shorthand anggota: ClassName : +method()
            if let Some(colon_pos) = line.find(':') {
                let cls_name = line[..colon_pos].trim();
                let member_text = line[colon_pos + 1..].trim();
                if !cls_name.is_empty() && !member_text.is_empty() {
                    let is_method = member_text.contains('(') && member_text.contains(')');
                    if !classes_map.contains_key(cls_name) {
                        classes_order.push(cls_name.to_string());
                        classes_map.insert(cls_name.to_string(), ClassEntity {
                            name: cls_name.to_string(),
                            members: Vec::new(),
                        });
                    }
                    if let Some(cls) = classes_map.get_mut(cls_name) {
                        cls.members.push(ClassMember {
                            is_method,
                            text: clean_markdown_inline(member_text),
                        });
                    }
                }
            }
        }

        if is_class {
            let classes = classes_order.into_iter().filter_map(|n| classes_map.remove(&n)).collect();
            Some(Self {
                title,
                classes,
                relations,
            })
        } else {
            None
        }
    }
}

// ============================================================================
// MERMAID ENTITY-RELATIONSHIP (ER) DIAGRAM PARSER & DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone)]
pub struct ErAttribute {
    pub data_type: String,
    pub name: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ErEntity {
    pub name: String,
    pub attributes: Vec<ErAttribute>,
}

#[derive(Debug, Clone)]
pub struct ErRelation {
    pub from: String,
    pub to: String,
    pub card_left: String,
    pub card_right: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct WidyaErDiagram {
    pub title: Option<String>,
    pub entities: Vec<ErEntity>,
    pub relations: Vec<ErRelation>,
}

impl WidyaErDiagram {
    pub fn parse(source: &str) -> Option<Self> {
        let mut is_er = false;
        let mut title = None;
        let mut entities_map: HashMap<String, ErEntity> = HashMap::new();
        let mut entities_order: Vec<String> = Vec::new();
        let mut relations: Vec<ErRelation> = Vec::new();

        let mut inside_entity: Option<String> = None;

        for raw_line in source.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            if line.starts_with("erDiagram") {
                is_er = true;
                continue;
            }

            if !is_er {
                continue;
            }

            if line.starts_with("title:") || line.starts_with("title ") {
                let t = line.trim_start_matches("title:").trim_start_matches("title").trim();
                title = Some(clean_markdown_inline(t));
                continue;
            }

            // Penutup blok entitas
            if line == "}" {
                inside_entity = None;
                continue;
            }

            // Di dalam blok entitas: atribut
            if let Some(ref ent_name) = inside_entity {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let data_type = parts[0].to_string();
                    let name = parts[1].to_string();
                    let mut keys = Vec::new();
                    for &p in &parts[2..] {
                        let up = p.to_uppercase();
                        if up == "PK" || up == "FK" || up == "UK" {
                            keys.push(up);
                        }
                    }
                    if let Some(ent) = entities_map.get_mut(ent_name) {
                        ent.attributes.push(ErAttribute {
                            data_type,
                            name,
                            keys,
                        });
                    }
                }
                continue;
            }

            // Pembuka blok entitas: ENTITY {
            if line.ends_with('{') {
                let name = line.trim_end_matches('{').trim().to_string();
                if !entities_map.contains_key(&name) {
                    entities_order.push(name.clone());
                    entities_map.insert(name.clone(), ErEntity {
                        name: name.clone(),
                        attributes: Vec::new(),
                    });
                }
                inside_entity = Some(name);
                continue;
            }

            // Relasi: ENTITY1 op ENTITY2 : label
            if line.contains(':') && (line.contains("--") || line.contains("..")) {
                let colon_parts: Vec<&str> = line.splitn(2, ':').collect();
                let left = colon_parts[0].trim();
                let label = if colon_parts.len() > 1 {
                    colon_parts[1].trim().trim_matches('"').to_string()
                } else {
                    String::new()
                };

                let tokens: Vec<&str> = left.split_whitespace().collect();
                if tokens.len() >= 3 {
                    let from = tokens[0].to_string();
                    let op = tokens[1];
                    let to = tokens[2].to_string();

                    if !entities_map.contains_key(&from) {
                        entities_order.push(from.clone());
                        entities_map.insert(from.clone(), ErEntity {
                            name: from.clone(),
                            attributes: Vec::new(),
                        });
                    }
                    if !entities_map.contains_key(&to) {
                        entities_order.push(to.clone());
                        entities_map.insert(to.clone(), ErEntity {
                            name: to.clone(),
                            attributes: Vec::new(),
                        });
                    }

                    relations.push(ErRelation {
                        from,
                        to,
                        card_left: op.chars().take(2).collect(),
                        card_right: op.chars().rev().take(2).collect(),
                        label,
                    });
                }
                continue;
            }

            // Entitas mandiri tanpa atribut: ENTITY
            if !line.contains(' ') && !line.is_empty() {
                let name = line.to_string();
                if !entities_map.contains_key(&name) {
                    entities_order.push(name.clone());
                    entities_map.insert(name.clone(), ErEntity {
                        name,
                        attributes: Vec::new(),
                    });
                }
            }
        }

        if is_er {
            let entities = entities_order.into_iter().filter_map(|n| entities_map.remove(&n)).collect();
            Some(Self {
                title,
                entities,
                relations,
            })
        } else {
            None
        }
    }
}

/// Mesin Pembangkit Diagram Vektor Mandiri Widya-Lang (Zero Dependency)
pub struct WidyaDiagramEngine;

impl WidyaDiagramEngine {
    /// Render Diagram Mermaid secara langsung menjadi elemen Vektor Asli PDF
    pub fn render_to_pdf(
        pdf: &mut PdfDocument,
        source: &str,
        margin_x: f32,
        current_y: f32,
        content_width: f32,
    ) -> f32 {
        let (_, page_height) = pdf.page_size.dimensions_points();

        // 1. Cek apakah ini Mermaid Sequence Diagram
        if let Some(seq_diag) = WidyaSequenceDiagram::parse(source) {
            return Self::render_sequence_diagram(
                pdf,
                &seq_diag,
                margin_x,
                current_y,
                content_width,
                page_height,
            );
        }

        // 2. Cek apakah ini Mermaid State Machine Diagram
        if let Some(state_diag) = WidyaStateDiagram::parse(source) {
            return Self::render_state_diagram(
                pdf,
                &state_diag,
                margin_x,
                current_y,
                content_width,
                page_height,
            );
        }

        // 3. Cek apakah ini Mermaid Class Diagram
        if let Some(class_diag) = WidyaClassDiagram::parse(source) {
            return Self::render_class_diagram(
                pdf,
                &class_diag,
                margin_x,
                current_y,
                content_width,
                page_height,
            );
        }

        // 4. Cek apakah ini Mermaid Entity-Relationship (ER) Diagram
        if let Some(er_diag) = WidyaErDiagram::parse(source) {
            return Self::render_er_diagram(
                pdf,
                &er_diag,
                margin_x,
                current_y,
                content_width,
                page_height,
            );
        }

        let diagram = WidyaDiagram::parse(source);

        // 4. Jika diagram memiliki Subgraph (Diagram Arsitektur Sistem)
        if !diagram.subgraphs.is_empty() {
            return Self::render_subgraph_architecture(
                pdf,
                &diagram,
                margin_x,
                current_y,
                content_width,
                page_height,
            );
        }

        // 3. Jika diagram flowchart sederhana tanpa Subgraph
        if !diagram.nodes.is_empty() {
            return Self::render_simple_flowchart(
                pdf,
                &diagram,
                margin_x,
                current_y,
                content_width,
                page_height,
            );
        }

        // Fallback: Jika tidak terurai, kembalikan posisi asal
        current_y
    }

    /// Render Diagram Arsitektur Berbasis Kartu Subgraph & Relasi Vektor
    fn render_subgraph_architecture(
        pdf: &mut PdfDocument,
        diagram: &WidyaDiagram,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
        page_height: f32,
    ) -> f32 {
        // Banner Judul Diagram
        if current_y < 90.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        let banner_h = 22.0;
        let page = pdf.current_page_mut();
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, true, (0.92, 0.95, 0.99));
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, false, (0.18, 0.42, 0.76));
        page.add_styled_line(margin_x, current_y - banner_h, margin_x, current_y, 3.5, (0.1, 0.45, 0.9));
        page.add_styled_text(
            "DIAGRAM ARSITEKTUR & ALUR KEDAULATAN SISTEM",
            margin_x + 12.0,
            current_y - 15.0,
            9.5,
            true,
            (0.08, 0.22, 0.48),
        );
        current_y -= banner_h + 8.0;

        // Tampilkan Alur Relasi jika ada edge antar Subgraph
        let mut rel_text = String::new();
        for (idx, edge) in diagram.edges.iter().enumerate() {
            if idx > 0 {
                rel_text.push_str("  |  ");
            }
            rel_text.push_str(&format!("{} -> {}", edge.from, edge.to));
        }

        if !rel_text.is_empty() {
            let rel_lines = wrap_text(&format!("Alur Relasi: {}", rel_text), content_width - 20.0, 8.0, false);
            for line in rel_lines {
                if current_y < 80.0 {
                    pdf.add_page();
                    current_y = page_height - 70.0;
                }
                pdf.current_page_mut().add_styled_text(&line, margin_x + 6.0, current_y - 8.0, 8.0, false, (0.35, 0.45, 0.55));
                current_y -= 12.0;
            }
            current_y -= 4.0;
        }

        let node_map: HashMap<String, &DiagramNode> = diagram.nodes.iter().map(|n| (n.id.clone(), n)).collect();

        // Render Tiap Kartu Subgraph
        let total_subgraphs = diagram.subgraphs.len();
        for (sg_idx, sg) in diagram.subgraphs.iter().enumerate() {
            // Hitung tinggi yang dibutuhkan kartu ini
            let header_h = 18.0;
            let mut items_lines: Vec<Vec<String>> = Vec::new();

            for node_id in &sg.node_ids {
                if let Some(node) = node_map.get(node_id) {
                    let wrapped = wrap_text(&node.label, content_width - 36.0, 8.5, false);
                    items_lines.push(wrapped);
                }
            }

            let mut body_h = 8.0;
            for lines in &items_lines {
                body_h += (lines.len() as f32 * 11.5) + 3.0;
            }
            let total_card_h = header_h + body_h;

            // Pemisah halaman jika kartu melebihi batas bawah
            if current_y - total_card_h < 75.0 {
                pdf.add_page();
                current_y = page_height - 70.0;
            }

            let page = pdf.current_page_mut();

            // 1. Header Kartu Subgraph
            page.add_rect(margin_x, current_y - header_h, content_width, header_h, true, (0.12, 0.28, 0.52));
            page.add_styled_line(margin_x, current_y - header_h, margin_x, current_y, 4.0, (0.20, 0.65, 0.95));
            page.add_styled_text(&sg.title, margin_x + 12.0, current_y - 12.5, 9.0, true, (1.0, 1.0, 1.0));

            // 2. Body Kartu Subgraph
            page.add_rect(margin_x, current_y - total_card_h, content_width, body_h, true, (0.975, 0.985, 1.0));
            page.add_rect(margin_x, current_y - total_card_h, content_width, total_card_h, false, (0.25, 0.45, 0.75));

            // 3. Butir Isi di dalam Subgraph
            let mut item_y = current_y - header_h - 11.0;
            for lines in items_lines {
                for (l_idx, line) in lines.iter().enumerate() {
                    let page = pdf.current_page_mut();
                    if l_idx == 0 {
                        page.add_styled_text("*", margin_x + 12.0, item_y, 9.0, true, (0.15, 0.45, 0.85));
                    }
                    if !line.is_empty() {
                        page.add_styled_text(line, margin_x + 22.0, item_y, 8.5, false, (0.15, 0.20, 0.25));
                    }
                    item_y -= 11.5;
                }
                item_y -= 3.0;
            }

            current_y -= total_card_h;

            // Panah vektor penghubung ke kartu berikutnya jika ada
            if sg_idx + 1 < total_subgraphs {
                current_y -= 4.0;
                let mid_x = margin_x + content_width / 2.0;
                pdf.current_page_mut().add_arrow(mid_x, current_y, mid_x, current_y - 12.0, 1.5, (0.2, 0.5, 0.85));
                current_y -= 16.0;
            }
        }

        current_y
    }

    /// Render Diagram Flowchart Node-Edge Sederhana
    fn render_simple_flowchart(
        pdf: &mut PdfDocument,
        diagram: &WidyaDiagram,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
        page_height: f32,
    ) -> f32 {
        if diagram.nodes.is_empty() {
            return current_y;
        }

        // Hitung in-degree dan layer topological untuk tiap node
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();

        for node in &diagram.nodes {
            in_degree.insert(node.id.clone(), 0);
            adj.insert(node.id.clone(), Vec::new());
        }

        for edge in &diagram.edges {
            *in_degree.entry(edge.to.clone()).or_insert(0) += 1;
            adj.entry(edge.from.clone()).or_default().push(edge.to.clone());
        }

        // Hitung level rank untuk tiap node
        let mut ranks: HashMap<String, usize> = HashMap::new();
        for node in &diagram.nodes {
            if in_degree.get(&node.id).copied().unwrap_or(0) == 0 {
                ranks.insert(node.id.clone(), 0);
            }
        }

        // Propagasi rank
        for _ in 0..diagram.nodes.len() {
            for edge in &diagram.edges {
                let from_rank = ranks.get(&edge.from).copied().unwrap_or(0);
                let cur_to = ranks.get(&edge.to).copied().unwrap_or(0);
                if from_rank + 1 > cur_to {
                    ranks.insert(edge.to.clone(), from_rank + 1);
                }
            }
        }

        // Kelompokkan node per rank
        let mut max_rank = 0;
        let mut rank_groups: HashMap<usize, Vec<&DiagramNode>> = HashMap::new();
        for node in &diagram.nodes {
            let r = ranks.get(&node.id).copied().unwrap_or(0);
            if r > max_rank {
                max_rank = r;
            }
            rank_groups.entry(r).or_default().push(node);
        }

        let layer_height = 42.0;
        let total_chart_h = (max_rank as f32 + 1.0) * layer_height + 20.0;

        if current_y - total_chart_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        // Simpan posisi tengah tiap node untuk menggambar edge nanti
        let mut node_positions: HashMap<String, (f32, f32, f32, f32)> = HashMap::new(); // (center_x, center_y, width, height)

        let node_h = 26.0;

        for r in 0..=max_rank {
            let layer_nodes = rank_groups.get(&r).cloned().unwrap_or_default();
            let count = layer_nodes.len();
            if count == 0 {
                continue;
            }

            let available_w = content_width - 20.0;
            let node_w = (available_w / count as f32 - 16.0).min(180.0).max(65.0);
            let total_layer_w = count as f32 * node_w + (count.saturating_sub(1) as f32 * 16.0);
            let start_x = margin_x + (content_width - total_layer_w) / 2.0;

            let row_y = current_y - (r as f32 * layer_height);

            for (i, node) in layer_nodes.iter().enumerate() {
                let x = start_x + (i as f32 * (node_w + 16.0));
                let page = pdf.current_page_mut();

                // Latar belakang dan border node
                let bg_color = match node.shape {
                    NodeShape::Diamond => (0.98, 0.95, 1.0),
                    NodeShape::Rounded => (0.94, 0.98, 0.95),
                    NodeShape::Rectangle => (0.93, 0.96, 1.0),
                };
                let border_color = match node.shape {
                    NodeShape::Diamond => (0.5, 0.2, 0.8),
                    NodeShape::Rounded => (0.1, 0.6, 0.3),
                    NodeShape::Rectangle => (0.18, 0.42, 0.76),
                };

                page.add_rect(x, row_y - node_h, node_w, node_h, true, bg_color);
                page.add_rect(x, row_y - node_h, node_w, node_h, false, border_color);

                // Teks label node (dipusatkan)
                let label_text = if node.label.chars().count() > 24 {
                    let trunc: String = node.label.chars().take(22).collect();
                    format!("{}..", trunc)
                } else {
                    node.label.clone()
                };

                let text_x = x + 6.0;
                let text_y = row_y - 16.5;
                page.add_styled_text(&label_text, text_x, text_y, 8.5, true, (0.08, 0.20, 0.45));

                node_positions.insert(node.id.clone(), (x + node_w / 2.0, row_y - node_h / 2.0, node_w, node_h));
            }
        }

        // Gambar Garis & Panah Edge antar node
        let page = pdf.current_page_mut();
        for edge in &diagram.edges {
            if let (Some(&(x1, y1, _w1, h1)), Some(&(x2, y2, _w2, h2))) = (node_positions.get(&edge.from), node_positions.get(&edge.to)) {
                let start_point = (x1, y1 - h1 / 2.0);
                let end_point = (x2, y2 + h2 / 2.0);

                if edge.has_arrow {
                    page.add_arrow(start_point.0, start_point.1, end_point.0, end_point.1, 1.2, (0.25, 0.45, 0.75));
                } else {
                    page.add_styled_line(start_point.0, start_point.1, end_point.0, end_point.1, 1.2, (0.25, 0.45, 0.75));
                }
            }
        }

        current_y -= total_chart_h;
        current_y
    }

    /// Render Diagram Urutan (Mermaid Sequence Diagram) ke Elemen Vektor Asli PDF
    fn render_sequence_diagram(
        pdf: &mut PdfDocument,
        diag: &WidyaSequenceDiagram,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
        page_height: f32,
    ) -> f32 {
        if diag.participants.is_empty() {
            return current_y;
        }

        let banner_h = 22.0;
        let box_h = 20.0;
        let msg_h = 24.0;
        let total_diag_h = banner_h + 10.0 + box_h + 10.0 + (diag.messages.len() as f32 * msg_h) + 12.0 + box_h + 14.0;

        if current_y - total_diag_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        // 1. Banner Judul Sequence Diagram
        let page = pdf.current_page_mut();
        let title_text = diag.title.as_deref().unwrap_or("DIAGRAM URUTAN (SEQUENCE DIAGRAM) & ALUR PESAN");
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, true, (0.92, 0.95, 0.99));
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, false, (0.18, 0.42, 0.76));
        page.add_styled_line(margin_x, current_y - banner_h, margin_x, current_y, 3.5, (0.1, 0.45, 0.9));
        page.add_styled_text(
            title_text,
            margin_x + 12.0,
            current_y - 15.0,
            9.5,
            true,
            (0.08, 0.22, 0.48),
        );
        current_y -= banner_h + 10.0;

        // 2. Hitung Posisi Partisipan
        let n = diag.participants.len();
        let col_w = content_width / n as f32;
        let box_w = (col_w - 12.0).min(110.0).max(45.0);

        let mut x_centers: HashMap<String, f32> = HashMap::new();
        for (i, p) in diag.participants.iter().enumerate() {
            let cx = margin_x + (i as f32 + 0.5) * col_w;
            x_centers.insert(p.id.clone(), cx);
        }

        let top_box_bottom_y = current_y - box_h;

        // Render Kotak Partisipan Bagian Atas
        for p in &diag.participants {
            if let Some(&cx) = x_centers.get(&p.id) {
                let bx = cx - box_w / 2.0;
                let page = pdf.current_page_mut();
                page.add_rect(bx, current_y - box_h, box_w, box_h, true, (0.12, 0.28, 0.52));
                page.add_rect(bx, current_y - box_h, box_w, box_h, false, (0.2, 0.45, 0.75));

                let label_text = if p.label.chars().count() > 16 {
                    let tr: String = p.label.chars().take(14).collect();
                    format!("{}..", tr)
                } else {
                    p.label.clone()
                };
                let tw = crate::office::pdf::styled_string_width(&label_text, 8.5, true, false);
                let tx = (cx - tw / 2.0).max(bx + 2.0);
                page.add_styled_text(&label_text, tx, current_y - 13.5, 8.5, true, (1.0, 1.0, 1.0));
            }
        }
        current_y -= box_h + 10.0;

        // Hitung Y akhir untuk lifelines
        let lifeline_start_y = top_box_bottom_y;
        let lifeline_end_y = current_y - (diag.messages.len() as f32 * msg_h) - 4.0;

        // 3. Render Garis Hidup Vertikal (Lifeline)
        for p in &diag.participants {
            if let Some(&cx) = x_centers.get(&p.id) {
                Self::draw_dashed_vertical_line(pdf.current_page_mut(), cx, lifeline_start_y, lifeline_end_y, 0.8, (0.65, 0.70, 0.78));
            }
        }

        // 4. Render Pesan Interaksi
        for msg in &diag.messages {
            let arrow_y = current_y - 12.0;
            if let (Some(&x1), Some(&x2)) = (x_centers.get(&msg.from), x_centers.get(&msg.to)) {
                let page = pdf.current_page_mut();

                // Teks pesan di atas garis panah
                if !msg.text.is_empty() {
                    let mid_x = (x1 + x2) / 2.0;
                    let tw = crate::office::pdf::styled_string_width(&msg.text, 8.0, false, false);
                    let tx = (mid_x - tw / 2.0).max(margin_x + 4.0).min(margin_x + content_width - tw - 4.0);
                    page.add_styled_text(&msg.text, tx, arrow_y + 3.0, 8.0, false, (0.12, 0.20, 0.35));
                }

                // Garis Panah
                let is_dashed = msg.arrow == SequenceArrowType::DashedArrow || msg.arrow == SequenceArrowType::DashedLine;
                let color = (0.2, 0.45, 0.75);

                if is_dashed {
                    Self::draw_dashed_horizontal_line(page, x1, x2, arrow_y, 1.0, color);
                } else {
                    page.add_styled_line(x1, arrow_y, x2, arrow_y, 1.0, color);
                }

                // Kepala Panah jika SolidArrow atau DashedArrow
                if msg.arrow == SequenceArrowType::SolidArrow || msg.arrow == SequenceArrowType::DashedArrow {
                    if x2 > x1 {
                        page.add_arrow(x2 - 8.0, arrow_y, x2, arrow_y, 1.0, color);
                    } else if x2 < x1 {
                        page.add_arrow(x2 + 8.0, arrow_y, x2, arrow_y, 1.0, color);
                    }
                }
            }
            current_y -= msg_h;
        }

        current_y -= 8.0;

        // 5. Render Kotak Partisipan Bagian Bawah
        for p in &diag.participants {
            if let Some(&cx) = x_centers.get(&p.id) {
                let bx = cx - box_w / 2.0;
                let page = pdf.current_page_mut();
                page.add_rect(bx, current_y - box_h, box_w, box_h, true, (0.92, 0.95, 0.99));
                page.add_rect(bx, current_y - box_h, box_w, box_h, false, (0.2, 0.45, 0.75));

                let label_text = if p.label.chars().count() > 16 {
                    let tr: String = p.label.chars().take(14).collect();
                    format!("{}..", tr)
                } else {
                    p.label.clone()
                };
                let tw = crate::office::pdf::styled_string_width(&label_text, 8.5, true, false);
                let tx = (cx - tw / 2.0).max(bx + 2.0);
                page.add_styled_text(&label_text, tx, current_y - 13.5, 8.5, true, (0.12, 0.28, 0.52));
            }
        }
        current_y -= box_h + 12.0;

        current_y
    }

    fn draw_dashed_horizontal_line(
        page: &mut PdfPage,
        x1: f32,
        x2: f32,
        y: f32,
        width: f32,
        color: (f32, f32, f32),
    ) {
        let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let dash_len = 4.0;
        let gap_len = 3.0;
        let mut cur = min_x;
        while cur < max_x {
            let next = (cur + dash_len).min(max_x);
            page.add_styled_line(cur, y, next, y, width, color);
            cur += dash_len + gap_len;
        }
    }

    fn draw_dashed_vertical_line(
        page: &mut PdfPage,
        x: f32,
        y_start: f32,
        y_end: f32,
        width: f32,
        color: (f32, f32, f32),
    ) {
        let (top_y, bot_y) = if y_start > y_end { (y_start, y_end) } else { (y_end, y_start) };
        let dash_len = 4.0;
        let gap_len = 3.0;
        let mut cur = top_y;
        while cur > bot_y {
            let next = (cur - dash_len).max(bot_y);
            page.add_styled_line(x, cur, x, next, width, color);
            cur -= dash_len + gap_len;
        }
    }

    /// Render Diagram Mesin Status (Mermaid stateDiagram-v2) ke Elemen Vektor PDF
    fn render_state_diagram(
        pdf: &mut PdfDocument,
        diag: &WidyaStateDiagram,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
        page_height: f32,
    ) -> f32 {
        if diag.states.is_empty() {
            return current_y;
        }

        let banner_h = 22.0;
        let layer_h = 44.0;
        let total_diag_h = banner_h + 12.0 + (diag.states.len() as f32 + 2.0) * layer_h;

        if current_y - total_diag_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        // Banner Judul
        let page = pdf.current_page_mut();
        let title_text = diag.title.as_deref().unwrap_or("DIAGRAM STATUS SISTEM (STATE MACHINE DIAGRAM)");
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, true, (0.92, 0.95, 0.99));
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, false, (0.18, 0.42, 0.76));
        page.add_styled_line(margin_x, current_y - banner_h, margin_x, current_y, 3.5, (0.1, 0.45, 0.9));
        page.add_styled_text(title_text, margin_x + 12.0, current_y - 15.0, 9.5, true, (0.08, 0.22, 0.48));
        current_y -= banner_h + 16.0;

        // Hitung posisi vertikal untuk tiap state
        let mid_x = margin_x + content_width / 2.0;
        let state_w = 140.0;
        let state_h = 24.0;

        let mut state_positions: HashMap<String, (f32, f32)> = HashMap::new();

        // 1. Initial State [*]
        let initial_y = current_y - 10.0;
        let page = pdf.current_page_mut();
        // Lingkaran awal hitam terisi kecil (diameter 12.0)
        page.add_rect(mid_x - 6.0, initial_y - 6.0, 12.0, 12.0, true, (0.15, 0.2, 0.3));
        state_positions.insert("[*]".to_string(), (mid_x, initial_y));
        current_y -= 36.0;

        // 2. States reguler
        for state_name in &diag.states {
            let sy = current_y - state_h / 2.0;
            let sx = mid_x - state_w / 2.0;
            let page = pdf.current_page_mut();

            // Kapsul state rounded/rect
            page.add_rect(sx, current_y - state_h, state_w, state_h, true, (0.94, 0.97, 1.0));
            page.add_rect(sx, current_y - state_h, state_w, state_h, false, (0.18, 0.45, 0.78));

            let tw = crate::office::pdf::styled_string_width(state_name, 9.0, true, false);
            let tx = mid_x - tw / 2.0;
            page.add_styled_text(state_name, tx, current_y - 16.0, 9.0, true, (0.10, 0.25, 0.50));

            state_positions.insert(state_name.clone(), (mid_x, sy));
            current_y -= layer_h;
        }

        // 3. Final State [*]_end
        let final_y = current_y - 10.0;
        let page = pdf.current_page_mut();
        // Lingkaran akhir: lingkaran luar border dan dalam terisi
        page.add_rect(mid_x - 8.0, final_y - 8.0, 16.0, 16.0, false, (0.15, 0.2, 0.3));
        page.add_rect(mid_x - 4.5, final_y - 4.5, 9.0, 9.0, true, (0.15, 0.2, 0.3));
        current_y -= 24.0;

        // 4. Gambar panah transisi
        let page = pdf.current_page_mut();
        for tr in &diag.transitions {
            let from_pos = state_positions.get(&tr.from);
            let to_pos = if tr.to == "[*]" {
                Some(&(mid_x, final_y))
            } else {
                state_positions.get(&tr.to)
            };

            if let (Some(&(x1, y1)), Some(&(x2, y2))) = (from_pos, to_pos) {
                if y1 > y2 {
                    // Panah ke bawah
                    let start_pt = (x1, y1 - state_h / 2.0);
                    let end_pt = (x2, y2 + state_h / 2.0);
                    page.add_arrow(start_pt.0, start_pt.1, end_pt.0, end_pt.1, 1.2, (0.2, 0.45, 0.75));

                    if let Some(ref lbl) = tr.label {
                        page.add_styled_text(lbl, (x1 + x2) / 2.0 + 8.0, (start_pt.1 + end_pt.1) / 2.0 - 3.0, 7.5, false, (0.35, 0.42, 0.52));
                    }
                } else {
                    // Transisi balik / menyamping
                    let start_pt = (x1 + state_w / 2.0, y1);
                    let end_pt = (x2 + state_w / 2.0, y2);
                    page.add_arrow(start_pt.0, start_pt.1, end_pt.0, end_pt.1, 1.0, (0.25, 0.45, 0.75));
                    if let Some(ref lbl) = tr.label {
                        page.add_styled_text(lbl, start_pt.0 + 6.0, (y1 + y2) / 2.0, 7.5, false, (0.35, 0.42, 0.52));
                    }
                }
            }
        }

        current_y -= 10.0;
        current_y
    }

    /// Render Diagram Kelas UML (Mermaid classDiagram) ke Elemen Vektor PDF
    fn render_class_diagram(
        pdf: &mut PdfDocument,
        diag: &WidyaClassDiagram,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
        page_height: f32,
    ) -> f32 {
        if diag.classes.is_empty() {
            return current_y;
        }

        let banner_h = 22.0;
        let mut total_diag_h = banner_h + 20.0;
        for c in &diag.classes {
            total_diag_h += 24.0 + (c.members.len() as f32 * 12.0) + 16.0;
        }

        if current_y - total_diag_h.min(300.0) < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        // Banner Judul
        let page = pdf.current_page_mut();
        let title_text = diag.title.as_deref().unwrap_or("DIAGRAM KELAS & MODEL DATA (CLASS DIAGRAM)");
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, true, (0.92, 0.95, 0.99));
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, false, (0.18, 0.42, 0.76));
        page.add_styled_line(margin_x, current_y - banner_h, margin_x, current_y, 3.5, (0.1, 0.45, 0.9));
        page.add_styled_text(title_text, margin_x + 12.0, current_y - 15.0, 9.5, true, (0.08, 0.22, 0.48));
        current_y -= banner_h + 14.0;

        let num_classes = diag.classes.len();
        let col_count = if num_classes >= 2 { 2 } else { 1 };
        let col_w = (content_width - (col_count as f32 - 1.0) * 16.0) / col_count as f32;

        let mut class_positions: HashMap<String, (f32, f32, f32, f32)> = HashMap::new(); // (x, y, w, h)

        let mut row_start_y = current_y;
        let mut max_row_h = 0.0f32;

        for (idx, cls) in diag.classes.iter().enumerate() {
            let col_idx = idx % col_count;
            if col_idx == 0 && idx > 0 {
                row_start_y -= max_row_h + 16.0;
                max_row_h = 0.0;
            }

            let bx = margin_x + (col_idx as f32 * (col_w + 16.0));
            let header_h = 20.0;
            let body_h = 6.0 + (cls.members.len() as f32 * 12.0).max(12.0);
            let total_box_h = header_h + body_h;

            if total_box_h > max_row_h {
                max_row_h = total_box_h;
            }

            if row_start_y - total_box_h < 75.0 {
                pdf.add_page();
                row_start_y = page_height - 70.0;
            }

            let page = pdf.current_page_mut();

            // 1. Header Kotak Kelas (Nama Kelas)
            page.add_rect(bx, row_start_y - header_h, col_w, header_h, true, (0.12, 0.28, 0.52));
            page.add_rect(bx, row_start_y - total_box_h, col_w, total_box_h, false, (0.2, 0.45, 0.75));

            let name_w = crate::office::pdf::styled_string_width(&cls.name, 9.0, true, false);
            let name_x = bx + (col_w - name_w) / 2.0;
            page.add_styled_text(&cls.name, name_x, row_start_y - 14.0, 9.0, true, (1.0, 1.0, 1.0));

            // 2. Body Kompartemen Anggota (Fields & Methods)
            page.add_rect(bx, row_start_y - total_box_h, col_w, body_h, true, (0.975, 0.985, 1.0));
            page.add_styled_line(bx, row_start_y - header_h, bx + col_w, row_start_y - header_h, 1.0, (0.2, 0.45, 0.75));

            let mut member_y = row_start_y - header_h - 11.0;
            for m in &cls.members {
                let color = if m.is_method { (0.05, 0.35, 0.55) } else { (0.15, 0.20, 0.25) };
                page.add_formatted_text(&m.text, bx + 8.0, member_y, 8.0, false, false, true, color);
                member_y -= 12.0;
            }

            class_positions.insert(cls.name.clone(), (bx, row_start_y, col_w, total_box_h));
        }

        current_y = row_start_y - max_row_h - 14.0;

        // 3. Gambar relasi antar kelas jika ada
        let page = pdf.current_page_mut();
        for rel in &diag.relations {
            if let (Some(&(x1, y1, w1, h1)), Some(&(x2, y2, w2, h2))) = (class_positions.get(&rel.from), class_positions.get(&rel.to)) {
                let p1 = (x1 + w1 / 2.0, y1 - h1);
                let p2 = (x2 + w2 / 2.0, y2);
                page.add_arrow(p1.0, p1.1, p2.0, p2.1, 1.2, (0.2, 0.45, 0.75));
            }
        }

        current_y
    }

    /// Render Diagram Entitas-Relasi (Mermaid erDiagram) Vektor Asli PDF
    fn render_er_diagram(
        pdf: &mut PdfDocument,
        diag: &WidyaErDiagram,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
        page_height: f32,
    ) -> f32 {
        if diag.entities.is_empty() {
            return current_y;
        }

        let banner_h = 22.0;
        let mut total_diag_h = banner_h + 20.0;
        for ent in &diag.entities {
            total_diag_h += 24.0 + (ent.attributes.len() as f32 * 13.0) + 16.0;
        }

        if current_y - total_diag_h.min(320.0) < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        // 1. Banner Judul
        let page = pdf.current_page_mut();
        let title_text = diag.title.as_ref().map(|t| format!("DIAGRAM ENTITAS-RELASI: {}", t)).unwrap_or_else(|| "DIAGRAM ENTITAS-RELASI BASIS DATA (ER DIAGRAM)".to_string());
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, true, (0.92, 0.95, 0.99));
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, false, (0.18, 0.42, 0.76));
        page.add_styled_line(margin_x, current_y - banner_h, margin_x, current_y, 3.5, (0.1, 0.45, 0.9));
        page.add_styled_text(&title_text, margin_x + 12.0, current_y - 15.0, 9.5, true, (0.08, 0.22, 0.48));
        current_y -= banner_h + 14.0;

        let num_entities = diag.entities.len();
        let col_count = if num_entities >= 2 { 2 } else { 1 };
        let col_w = (content_width - (col_count as f32 - 1.0) * 16.0) / col_count as f32;

        let mut entity_positions: HashMap<String, (f32, f32, f32, f32)> = HashMap::new();
        let mut row_start_y = current_y;
        let mut max_row_h = 0.0f32;

        for (idx, ent) in diag.entities.iter().enumerate() {
            let col_idx = idx % col_count;
            if col_idx == 0 && idx > 0 {
                row_start_y -= max_row_h + 16.0;
                max_row_h = 0.0;
            }

            let bx = margin_x + col_idx as f32 * (col_w + 16.0);
            let header_h = 22.0;
            let attr_h = 13.0;
            let body_h = (ent.attributes.len() as f32 * attr_h).max(18.0) + 8.0;
            let total_box_h = header_h + body_h;

            if total_box_h > max_row_h {
                max_row_h = total_box_h;
            }

            if row_start_y - total_box_h < 75.0 {
                pdf.add_page();
                row_start_y = page_height - 70.0;
            }

            let page = pdf.current_page_mut();

            // Header Tabel Entitas
            page.add_rect(bx, row_start_y - header_h, col_w, header_h, true, (0.10, 0.22, 0.45));
            page.add_rect(bx, row_start_y - total_box_h, col_w, total_box_h, false, (0.18, 0.35, 0.65));

            let name_text = format!("[TABEL] {}", ent.name);
            let name_w = crate::office::pdf::styled_string_width(&name_text, 9.0, true, false);
            let name_x = bx + (col_w - name_w) / 2.0;
            page.add_styled_text(&name_text, name_x, row_start_y - 14.5, 9.0, true, (1.0, 1.0, 1.0));

            // Badan Kotak Atribut
            page.add_rect(bx, row_start_y - total_box_h, col_w, body_h, true, (0.98, 0.99, 1.0));
            page.add_styled_line(bx, row_start_y - header_h, bx + col_w, row_start_y - header_h, 1.0, (0.18, 0.35, 0.65));

            let mut attr_y = row_start_y - header_h - 11.0;
            for attr in &ent.attributes {
                // Kolom 1: Tipe data
                page.add_formatted_text(&attr.data_type, bx + 6.0, attr_y, 7.5, false, true, true, (0.25, 0.45, 0.55));

                // Kolom 2: Nama field
                let name_offset = 60.0;
                page.add_formatted_text(&attr.name, bx + name_offset, attr_y, 8.0, true, false, false, (0.12, 0.16, 0.22));

                // Kolom 3: Key badges (PK, FK, UK)
                let mut badge_x = bx + col_w - 24.0;
                for k in &attr.keys {
                    let badge_color = match k.as_str() {
                        "PK" => (0.85, 0.60, 0.10),
                        "FK" => (0.15, 0.45, 0.85),
                        _ => (0.10, 0.60, 0.35),
                    };
                    page.add_rect(badge_x, attr_y - 2.0, 18.0, 9.0, true, (0.92, 0.94, 0.98));
                    page.add_styled_text(k, badge_x + 2.0, attr_y, 6.5, true, badge_color);
                    badge_x -= 22.0;
                }

                attr_y -= attr_h;
            }

            entity_positions.insert(ent.name.clone(), (bx, row_start_y, col_w, total_box_h));
        }

        current_y = row_start_y - max_row_h - 14.0;

        // 3. Garis Relasi Antar Entitas
        let page = pdf.current_page_mut();
        for rel in &diag.relations {
            if let (Some(&(x1, y1, w1, h1)), Some(&(x2, y2, w2, h2))) = (entity_positions.get(&rel.from), entity_positions.get(&rel.to)) {
                let p1 = (x1 + w1 / 2.0, y1 - h1);
                let p2 = (x2 + w2 / 2.0, y2);
                page.add_arrow(p1.0, p1.1, p2.0, p2.1, 1.2, (0.15, 0.45, 0.80));

                if !rel.label.is_empty() {
                    let mid_x = (p1.0 + p2.0) / 2.0;
                    let mid_y = (p1.1 + p2.1) / 2.0;
                    page.add_rect(mid_x - 22.0, mid_y - 5.0, 44.0, 10.0, true, (1.0, 1.0, 1.0));
                    page.add_styled_text(&rel.label, mid_x - 18.0, mid_y - 2.0, 7.0, false, (0.2, 0.3, 0.4));
                }
            }
        }

        current_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mermaid_flowchart_subgraph() {
        let code = r#"
        flowchart TD
            subgraph Pilar_1["Pilar I: Kompiler & Linker Mandiri"]
                P1_1["Direct ELF-64 Binary Emitter (`src/compiler/pure_emitter.rs`)"]
                P1_2["Direct x86_64 Machine Code Generator (Zero GCC/Clang/ld)"]
            end

            subgraph Pilar_2["Pilar II: Pustaka Standar Murni"]
                P2_1["Kriptografi Murni"]
            end

            Pilar_2 --> Pilar_1
        "#;

        let diagram = WidyaDiagram::parse(code);
        assert_eq!(diagram.direction, DiagramDirection::TopToBottom);
        assert_eq!(diagram.subgraphs.len(), 2);
        assert_eq!(diagram.subgraphs[0].title, "Pilar I: Kompiler & Linker Mandiri");
        assert_eq!(diagram.subgraphs[0].node_ids.len(), 2);
        assert_eq!(diagram.edges.len(), 1);
        assert_eq!(diagram.edges[0].from, "Pilar_2");
        assert_eq!(diagram.edges[0].to, "Pilar_1");
    }

    #[test]
    fn test_parse_simple_flowchart() {
        let code = r#"
        flowchart LR
            A["Mulai"] --> B["Proses Data"]
            B --> C["Selesai"]
        "#;

        let diagram = WidyaDiagram::parse(code);
        assert_eq!(diagram.direction, DiagramDirection::LeftToRight);
        assert_eq!(diagram.nodes.len(), 3);
        assert_eq!(diagram.edges.len(), 2);
        assert_eq!(diagram.nodes[0].label, "Mulai");
    }

    #[test]
    fn test_parse_and_render_sequence_diagram() {
        let code = r#"
        sequenceDiagram
            title: Alur Autentikasi Pengguna
            participant Klien as Aplikasi Web
            participant Auth as Server Autentikasi
            participant DB as Basis Data Widya

            Klien->>Auth: POST /login (username, password)
            Auth->>DB: Cari identitas & hash argon2
            DB-->>Auth: Data akun terverifikasi
            Auth-->>Klien: JWT Token 200 OK
        "#;

        let seq = WidyaSequenceDiagram::parse(code);
        assert!(seq.is_some(), "Sequence diagram harus berhasil di-parse");
        let diag = seq.unwrap();
        assert_eq!(diag.title.as_deref(), Some("Alur Autentikasi Pengguna"));
        assert_eq!(diag.participants.len(), 3);
        assert_eq!(diag.participants[0].id, "Klien");
        assert_eq!(diag.participants[0].label, "Aplikasi Web");
        assert_eq!(diag.participants[2].id, "DB");
        assert_eq!(diag.participants[2].label, "Basis Data Widya");
        assert_eq!(diag.messages.len(), 4);
        assert_eq!(diag.messages[0].from, "Klien");
        assert_eq!(diag.messages[0].to, "Auth");
        assert_eq!(diag.messages[0].arrow, SequenceArrowType::SolidArrow);
        assert_eq!(diag.messages[2].arrow, SequenceArrowType::DashedArrow);

        // Uji rendering ke PDF
        let mut pdf = PdfDocument::new("Uji Sequence Diagram");
        let final_y = WidyaDiagramEngine::render_to_pdf(&mut pdf, code, 50.0, 750.0, 480.0);
        assert!(final_y < 750.0, "Y position harus berkurang setelah render");
        let bytes = pdf.generate_pdf_bytes();
        assert!(!bytes.is_empty(), "Bytes PDF hasil render sequence diagram harus valid");
    }

    #[test]
    fn test_parse_and_render_er_diagram() {
        let code = r#"
        erDiagram
            title: Skema Basis Data Transaksi
            PELANGGAN ||--o{ PESANAN : "membuat"
            PESANAN ||--|{ DETAIL_PESANAN : "memuat"

            PELANGGAN {
                string id_pelanggan PK
                string nama
                string email UK
            }

            PESANAN {
                string id_pesanan PK
                string id_pelanggan FK
                float total_harga
            }
        "#;

        let er = WidyaErDiagram::parse(code);
        assert!(er.is_some(), "ER Diagram harus berhasil di-parse");
        let diag = er.unwrap();
        assert_eq!(diag.title.as_deref(), Some("Skema Basis Data Transaksi"));
        assert_eq!(diag.entities.len(), 3);
        assert_eq!(diag.entities[0].name, "PELANGGAN");
        assert_eq!(diag.entities[0].attributes.len(), 3);
        assert_eq!(diag.entities[0].attributes[0].keys, vec!["PK"]);
        assert_eq!(diag.relations.len(), 2);
        assert_eq!(diag.relations[0].from, "PELANGGAN");
        assert_eq!(diag.relations[0].to, "PESANAN");
        assert_eq!(diag.relations[0].label, "membuat");

        let mut pdf = PdfDocument::new("Uji ER Diagram");
        let final_y = WidyaDiagramEngine::render_to_pdf(&mut pdf, code, 50.0, 750.0, 480.0);
        assert!(final_y < 750.0);
        let bytes = pdf.generate_pdf_bytes();
        assert!(!bytes.is_empty());
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("PELANGGAN"));
        assert!(text.contains("PESANAN"));
        assert!(text.contains("membuat"));
    }
}
