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
                    let raw_to_str = parts[1].trim();

                    let (edge_label, to_str) = if raw_to_str.starts_with('|') {
                        if let Some(end_bar) = raw_to_str[1..].find('|') {
                            let lbl = raw_to_str[1..1 + end_bar].trim().trim_matches('"');
                            (Some(lbl.to_string()), raw_to_str[1 + end_bar + 1..].trim())
                        } else {
                            (None, raw_to_str)
                        }
                    } else {
                        (None, raw_to_str)
                    };

                    let from_node = Self::extract_or_create_node(from_str, &current_subgraph);
                    let to_node = Self::extract_or_create_node(to_str, &current_subgraph);

                    // Daftarkan node jika belum ada
                    Self::register_node(&mut nodes_map, &mut nodes_order, &mut subgraphs, from_node.clone());
                    Self::register_node(&mut nodes_map, &mut nodes_order, &mut subgraphs, to_node.clone());

                    edges.push(DiagramEdge {
                        from: from_node.id,
                        to: to_node.id,
                        label: edge_label,
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
        // ID(["Label"]) atau ID([Label]) - stadium shape
        if let Some(b_start) = trimmed.find("([") {
            if trimmed.ends_with("])") {
                let id = trimmed[..b_start].trim().to_string();
                let mut label = trimmed[b_start + 2..trimmed.len() - 2].trim();
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
        // ID((Label)) - circle shape
        if let Some(b_start) = trimmed.find("((") {
            if trimmed.ends_with("))") {
                let id = trimmed[..b_start].trim().to_string();
                let mut label = trimmed[b_start + 2..trimmed.len() - 2].trim();
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

/// Irisan Data pada Pie Chart
#[derive(Debug, Clone)]
pub struct PieSlice {
    pub label: String,
    pub value: f64,
}

/// Struktur Data Diagram Lingkaran (Mermaid pie)
#[derive(Debug, Clone)]
pub struct WidyaPieChart {
    pub title: Option<String>,
    pub show_data: bool,
    pub slices: Vec<PieSlice>,
}

impl WidyaPieChart {
    pub fn parse(source: &str) -> Option<Self> {
        let mut is_pie = false;
        let mut title = None;
        let mut show_data = false;
        let mut slices = Vec::new();

        for raw_line in source.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            if line.starts_with("pie") {
                is_pie = true;
                if line.contains("showData") {
                    show_data = true;
                }
                if let Some(pos) = line.find("title") {
                    let t = line[pos + 5..].trim();
                    if !t.is_empty() {
                        title = Some(clean_markdown_inline(t.trim_matches('"').trim_matches('\'')));
                    }
                }
                continue;
            }

            if !is_pie {
                continue;
            }

            if line.starts_with("title ") || line.starts_with("title:") {
                let t = line.trim_start_matches("title:").trim_start_matches("title").trim();
                title = Some(clean_markdown_inline(t.trim_matches('"').trim_matches('\'')));
                continue;
            }

            if line.contains(':') {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                let label = parts[0].trim().trim_matches('"').trim_matches('\'').trim();
                let val_str = parts[1].trim();
                if let Ok(val) = val_str.parse::<f64>() {
                    if !label.is_empty() && val > 0.0 {
                        slices.push(PieSlice {
                            label: label.to_string(),
                            value: val,
                        });
                    }
                }
            }
        }

        if is_pie && !slices.is_empty() {
            Some(Self {
                title,
                show_data,
                slices,
            })
        } else {
            None
        }
    }
}

/// Status / Tingkat Kepentingan Tugas Gantt
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GanttStatus {
    Done,
    Active,
    Crit,
    Normal,
}

/// Item Tugas dalam Gantt Chart
#[derive(Debug, Clone)]
pub struct GanttTask {
    pub name: String,
    pub section: String,
    pub status: GanttStatus,
    pub start: f32,
    pub duration: f32,
}

/// Struktur Data Diagram Jadwal (Mermaid gantt)
#[derive(Debug, Clone)]
pub struct WidyaGanttChart {
    pub title: Option<String>,
    pub sections: Vec<String>,
    pub tasks: Vec<GanttTask>,
}

impl WidyaGanttChart {
    pub fn parse(source: &str) -> Option<Self> {
        let mut is_gantt = false;
        let mut title = None;
        let mut current_section = String::from("Tugas");
        let mut sections = Vec::new();
        let mut tasks = Vec::new();
        let mut cursor_time = 0.0f32;

        for raw_line in source.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }

            if line.starts_with("gantt") {
                is_gantt = true;
                continue;
            }

            if !is_gantt {
                continue;
            }

            if line.starts_with("title ") || line.starts_with("title:") {
                let t = line.trim_start_matches("title:").trim_start_matches("title").trim();
                title = Some(clean_markdown_inline(t.trim_matches('"').trim_matches('\'')));
                continue;
            }

            if line.starts_with("dateFormat") || line.starts_with("axisFormat") || line.starts_with("excludes") {
                continue;
            }

            if line.starts_with("section ") || line.starts_with("section:") {
                let sec = line.trim_start_matches("section:").trim_start_matches("section").trim();
                current_section = sec.to_string();
                if !sections.contains(&current_section) {
                    sections.push(current_section.clone());
                }
                continue;
            }

            if line.contains(':') {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                let task_name = parts[0].trim();
                let meta = parts[1].trim();

                let mut status = GanttStatus::Normal;
                let start_val = cursor_time;
                let mut dur_val = 5.0f32;

                let tokens: Vec<&str> = meta.split(',').map(|s| s.trim()).collect();
                for tok in &tokens {
                    let low = tok.to_lowercase();
                    if low == "done" {
                        status = GanttStatus::Done;
                    } else if low == "active" {
                        status = GanttStatus::Active;
                    } else if low == "crit" {
                        status = GanttStatus::Crit;
                    } else if low.ends_with('d') {
                        if let Ok(d) = low.trim_end_matches('d').parse::<f32>() {
                            dur_val = d.max(1.0);
                        }
                    } else if low.ends_with('w') {
                        if let Ok(w) = low.trim_end_matches('w').parse::<f32>() {
                            dur_val = (w * 7.0).max(1.0);
                        }
                    } else if let Ok(num) = tok.parse::<f32>() {
                        dur_val = num.max(1.0);
                    }
                }

                if !sections.contains(&current_section) {
                    sections.push(current_section.clone());
                }

                tasks.push(GanttTask {
                    name: task_name.to_string(),
                    section: current_section.clone(),
                    status,
                    start: start_val,
                    duration: dur_val,
                });

                cursor_time += dur_val.min(10.0);
            }
        }

        if is_gantt && !tasks.is_empty() {
            Some(Self {
                title,
                sections,
                tasks,
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

        // 5. Cek apakah ini Mermaid Pie Chart
        if let Some(pie_diag) = WidyaPieChart::parse(source) {
            return Self::render_pie_chart(
                pdf,
                &pie_diag,
                margin_x,
                current_y,
                content_width,
                page_height,
            );
        }

        // 6. Cek apakah ini Mermaid Gantt Chart
        if let Some(gantt_diag) = WidyaGanttChart::parse(source) {
            return Self::render_gantt_chart(
                pdf,
                &gantt_diag,
                margin_x,
                current_y,
                content_width,
                page_height,
            );
        }

        let diagram = WidyaDiagram::parse(source);

        if !diagram.nodes.is_empty() {
            return Self::render_flowchart(
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

    /// Render Diagram Alir (Flowchart) Mermaid dengan Tampilan Asli Mermaid.js
    fn render_flowchart(
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

        // 1. Hitung In-Degree dan Relasi Topological Sort untuk Rank Layering
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

        let mut ranks: HashMap<String, usize> = HashMap::new();
        for node in &diagram.nodes {
            if in_degree.get(&node.id).copied().unwrap_or(0) == 0 {
                ranks.insert(node.id.clone(), 0);
            }
        }

        // Propagasi rank (panjang lintasan terjauh)
        for _ in 0..diagram.nodes.len() {
            for edge in &diagram.edges {
                let from_rank = ranks.get(&edge.from).copied().unwrap_or(0);
                let cur_to = ranks.get(&edge.to).copied().unwrap_or(0);
                if from_rank + 1 > cur_to {
                    ranks.insert(edge.to.clone(), from_rank + 1);
                }
            }
        }

        for node in &diagram.nodes {
            ranks.entry(node.id.clone()).or_insert(0);
        }

        let mut max_rank = 0;
        let mut rank_groups: HashMap<usize, Vec<&DiagramNode>> = HashMap::new();
        for node in &diagram.nodes {
            let r = ranks.get(&node.id).copied().unwrap_or(0);
            if r > max_rank {
                max_rank = r;
            }
            rank_groups.entry(r).or_default().push(node);
        }

        // 2. Dimensi Simpul Dinamis & Bebas Pemotongan Kalimat
        let is_lr = diagram.direction == DiagramDirection::LeftToRight;
        let font_size = 8.0;
        let available_w = content_width - 16.0;

        // Hitung baris teks dan dimensi untuk setiap simpul (node_id -> (lines, node_w, node_h))
        let mut node_dim_info: HashMap<String, (Vec<String>, f32, f32)> = HashMap::new();
        let mut layer_heights: HashMap<usize, f32> = HashMap::new();
        let mut layer_widths: HashMap<usize, f32> = HashMap::new();

        if is_lr {
            let num_cols = max_rank + 1;
            let col_w = (content_width / num_cols as f32).min(150.0).max(75.0);
            let node_w = (col_w - 14.0).max(60.0);

            for r in 0..=max_rank {
                let layer_nodes = rank_groups.get(&r).cloned().unwrap_or_default();
                for node in &layer_nodes {
                    let lines = wrap_text(&node.label, node_w - 12.0, font_size, true);
                    let line_count = lines.len().max(1);
                    let nh = match line_count {
                        1 => 26.0,
                        2 => 38.0,
                        _ => 24.0 + (line_count as f32) * 11.5,
                    };
                    node_dim_info.insert(node.id.clone(), (lines, node_w, nh));
                }
            }
        } else {
            for r in 0..=max_rank {
                let layer_nodes = rank_groups.get(&r).cloned().unwrap_or_default();
                let count = layer_nodes.len();
                if count == 0 {
                    continue;
                }

                // Tentukan lebar simpul per lapisan agar proporsional dan teks tidak terpotong
                let layer_node_w = if count == 1 {
                    let node = layer_nodes[0];
                    let full_tw = crate::office::pdf::styled_string_width(&node.label, font_size, true, false);
                    let desired = (full_tw + 28.0).max(140.0);
                    desired.min(available_w.min(380.0))
                } else if count == 2 {
                    (available_w / 2.0 - 16.0).min(220.0).max(95.0)
                } else {
                    (available_w / count as f32 - 12.0).max(65.0)
                };

                layer_widths.insert(r, layer_node_w);

                let mut max_nh: f32 = 26.0;
                for node in &layer_nodes {
                    let inner_w = layer_node_w - 14.0;
                    let lines = wrap_text(&node.label, inner_w, font_size, true);
                    let line_count = lines.len().max(1);
                    let nh = match line_count {
                        1 => 26.0,
                        2 => 38.0,
                        _ => 24.0 + (line_count as f32) * 11.5,
                    };
                    if nh > max_nh {
                        max_nh = nh;
                    }
                    node_dim_info.insert(node.id.clone(), (lines, layer_node_w, nh));
                }
                layer_heights.insert(r, max_nh);
            }
        }

        // 3. Hitung Total Tinggi Diagram & Alokasi Halaman
        let arrow_gap = 20.0;
        let total_chart_h = if is_lr {
            let max_col_h = (0..=max_rank)
                .map(|r| {
                    let layer_nodes = rank_groups.get(&r).cloned().unwrap_or_default();
                    let sum_h: f32 = layer_nodes
                        .iter()
                        .map(|n| node_dim_info.get(&n.id).map(|info| info.2).unwrap_or(26.0) + 14.0)
                        .sum();
                    sum_h
                })
                .fold(60.0f32, f32::max);
            (max_col_h + 24.0).max(75.0)
        } else {
            let mut sum_h: f32 = 16.0;
            for r in 0..=max_rank {
                sum_h += layer_heights.get(&r).copied().unwrap_or(26.0);
                if r < max_rank {
                    sum_h += arrow_gap;
                }
            }
            sum_h + 16.0
        };

        if current_y - total_chart_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        let mut node_positions: HashMap<String, (f32, f32, f32, f32)> = HashMap::new(); // (x, y, w, h)

        // 4. Tata Letak Koordinat Simpul (Topological Grid)
        if is_lr {
            let num_cols = max_rank + 1;
            let col_w = (content_width / num_cols as f32).min(150.0).max(75.0);
            let total_w = num_cols as f32 * col_w;
            let start_x = margin_x + (content_width - total_w).max(0.0) / 2.0;

            for r in 0..=max_rank {
                let layer_nodes = rank_groups.get(&r).cloned().unwrap_or_default();
                let count = layer_nodes.len();
                if count == 0 {
                    continue;
                }
                let total_col_h: f32 = layer_nodes
                    .iter()
                    .map(|n| node_dim_info.get(&n.id).map(|i| i.2).unwrap_or(26.0))
                    .sum::<f32>()
                    + (count.saturating_sub(1) as f32 * 14.0);
                let col_start_y = current_y - (total_chart_h - total_col_h) / 2.0;

                let mut cur_y = col_start_y;
                for node in &layer_nodes {
                    let (_, nw, nh) = node_dim_info.get(&node.id).cloned().unwrap_or((Vec::new(), 70.0, 26.0));
                    let col_x = start_x + (r as f32 * col_w) + (col_w - nw) / 2.0;
                    let y = cur_y - nh;
                    node_positions.insert(node.id.clone(), (col_x, y, nw, nh));
                    cur_y = y - 14.0;
                }
            }
        } else {
            let mut cur_layer_y = current_y - 8.0;
            for r in 0..=max_rank {
                let layer_nodes = rank_groups.get(&r).cloned().unwrap_or_default();
                let count = layer_nodes.len();
                if count == 0 {
                    continue;
                }

                let layer_h = layer_heights.get(&r).copied().unwrap_or(26.0);
                let row_y = cur_layer_y - layer_h;

                let layer_node_w = layer_widths.get(&r).copied().unwrap_or(120.0);
                let total_layer_w = count as f32 * layer_node_w + (count.saturating_sub(1) as f32 * 14.0);
                let start_x = margin_x + (content_width - total_layer_w) / 2.0;

                for (i, node) in layer_nodes.iter().enumerate() {
                    let x = start_x + (i as f32 * (layer_node_w + 14.0));
                    node_positions.insert(node.id.clone(), (x, row_y, layer_node_w, layer_h));
                }

                cur_layer_y = row_y - arrow_gap;
            }
        }

        // 5. Render Kontainer Subgraph jika Ada (Gaya Boks Mermaid)
        for sg in &diagram.subgraphs {
            let mut min_x = f32::MAX;
            let mut min_y = f32::MAX;
            let mut max_x = f32::MIN;
            let mut max_y = f32::MIN;
            let mut has_nodes = false;

            for nid in &sg.node_ids {
                if let Some(&(nx, ny, nw, nh)) = node_positions.get(nid) {
                    has_nodes = true;
                    if nx < min_x { min_x = nx; }
                    if ny < min_y { min_y = ny; }
                    if nx + nw > max_x { max_x = nx + nw; }
                    if ny + nh > max_y { max_y = ny + nh; }
                }
            }

            if has_nodes {
                let pad_x = 10.0;
                let pad_y_bottom = 8.0;
                let pad_y_top = 18.0;
                let sg_x = (min_x - pad_x).max(margin_x);
                let sg_y = min_y - pad_y_bottom;
                let sg_w = (max_x - min_x + 2.0 * pad_x).min(content_width);
                let sg_h = max_y - min_y + pad_y_bottom + pad_y_top;

                let page = pdf.current_page_mut();
                page.add_rounded_rect(sg_x, sg_y, sg_w, sg_h, 6.0, true, (0.975, 0.98, 0.988));
                page.add_rounded_rect(sg_x, sg_y, sg_w, sg_h, 6.0, false, (0.78, 0.82, 0.88));
                page.add_styled_text(&sg.title, sg_x + 8.0, sg_y + sg_h - 12.0, 8.5, true, (0.18, 0.24, 0.35));
            }
        }

        // 6. Render Node & Teks Penuh Tanpa Pemotongan
        for node in &diagram.nodes {
            if let Some(&(x, y, w, h)) = node_positions.get(&node.id) {
                let page = pdf.current_page_mut();
                let cx = x + w / 2.0;
                let cy = y + h / 2.0;

                match node.shape {
                    NodeShape::Diamond => {
                        let poly = vec![
                            (cx, y + h),
                            (x + w, cy),
                            (cx, y),
                            (x, cy),
                        ];
                        page.add_polygon(poly.clone(), true, (1.0, 0.97, 0.88));
                        page.add_polygon(poly, false, (0.95, 0.65, 0.15));
                    }
                    NodeShape::Rounded => {
                        let radius = (h / 2.0).min(12.0);
                        page.add_rounded_rect(x, y, w, h, radius, true, (0.91, 0.96, 0.92));
                        page.add_rounded_rect(x, y, w, h, radius, false, (0.30, 0.69, 0.31));
                    }
                    NodeShape::Rectangle => {
                        page.add_rounded_rect(x, y, w, h, 4.5, true, (0.925, 0.925, 1.0));
                        page.add_rounded_rect(x, y, w, h, 4.5, false, (0.576, 0.439, 0.859));
                    }
                }

                let text_color = match node.shape {
                    NodeShape::Diamond => (0.20, 0.20, 0.20),
                    NodeShape::Rounded => (0.12, 0.32, 0.16),
                    NodeShape::Rectangle => (0.18, 0.18, 0.22),
                };

                let fallback = vec![node.label.clone()];
                let lines = if let Some(info) = node_dim_info.get(&node.id) {
                    &info.0
                } else {
                    &fallback
                };
                let line_count = lines.len().max(1);
                let line_spacing = 10.5;
                let total_text_h = (line_count.saturating_sub(1) as f32) * line_spacing;
                let start_ty = cy + (total_text_h / 2.0) - 2.8;

                for (idx, line_str) in lines.iter().enumerate() {
                    let tw = crate::office::pdf::styled_string_width(line_str, font_size, true, false);
                    let tx = (cx - tw / 2.0).max(x + 3.0);
                    let ty = start_ty - (idx as f32 * line_spacing);
                    page.add_styled_text(line_str, tx, ty, font_size, true, text_color);
                }
            }
        }

        // 6. Render Panah Pengarah dan Label Relasi
        let page = pdf.current_page_mut();
        for edge in &diagram.edges {
            if let (Some(&(x1, y1, w1, h1)), Some(&(x2, y2, w2, h2))) =
                (node_positions.get(&edge.from), node_positions.get(&edge.to))
            {
                let edge_color = (0.28, 0.30, 0.35);
                let start_pt;
                let end_pt;

                if is_lr {
                    start_pt = (x1 + w1, y1 + h1 / 2.0);
                    end_pt = (x2, y2 + h2 / 2.0);
                } else {
                    if y1 > y2 {
                        start_pt = (x1 + w1 / 2.0, y1);
                        end_pt = (x2 + w2 / 2.0, y2 + h2);
                    } else if (y1 - y2).abs() < 5.0 {
                        start_pt = (x1 + w1, y1 + h1 / 2.0);
                        end_pt = (x2, y2 + h2 / 2.0);
                    } else {
                        start_pt = (x1 + w1 / 2.0, y1 + h1);
                        end_pt = (x2 + w2 / 2.0, y2);
                    }
                }

                if edge.has_arrow {
                    page.add_arrow(start_pt.0, start_pt.1, end_pt.0, end_pt.1, 1.2, edge_color);
                } else {
                    page.add_styled_line(start_pt.0, start_pt.1, end_pt.0, end_pt.1, 1.2, edge_color);
                }

                if let Some(ref lbl) = edge.label {
                    let mid_x = (start_pt.0 + end_pt.0) / 2.0;
                    let mid_y = (start_pt.1 + end_pt.1) / 2.0;
                    let l_clean = clean_markdown_inline(lbl);
                    let lw = crate::office::pdf::styled_string_width(&l_clean, 7.5, true, false);
                    let badge_w = lw + 8.0;
                    let badge_h = 13.0;
                    let bx = mid_x - badge_w / 2.0;
                    let by = mid_y - badge_h / 2.0;

                    page.add_rounded_rect(bx, by, badge_w, badge_h, 2.5, true, (1.0, 1.0, 1.0));
                    page.add_rounded_rect(bx, by, badge_w, badge_h, 2.5, false, (0.80, 0.84, 0.88));
                    page.add_styled_text(&l_clean, bx + 4.0, by + 3.0, 7.5, true, (0.25, 0.28, 0.35));
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

        let title_h = if diag.title.is_some() { 22.0 } else { 0.0 };
        let box_h = 20.0;
        let msg_h = 24.0;
        let total_diag_h = title_h + box_h + 10.0 + (diag.messages.len() as f32 * msg_h) + 12.0 + box_h + 14.0;

        if current_y - total_diag_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        if let Some(ref t) = diag.title {
            let page = pdf.current_page_mut();
            let tw = crate::office::pdf::styled_string_width(t, 9.5, true, false);
            let tx = margin_x + (content_width - tw) / 2.0;
            page.add_styled_text(t, tx, current_y - 14.0, 9.5, true, (0.2, 0.22, 0.28));
            current_y -= title_h;
        }

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
                page.add_rounded_rect(bx, current_y - box_h, box_w, box_h, 4.0, true, (0.925, 0.925, 1.0));
                page.add_rounded_rect(bx, current_y - box_h, box_w, box_h, 4.0, false, (0.576, 0.439, 0.859));

                let req_w = crate::office::pdf::styled_string_width(&p.label, 8.5, true, false);
                let (font_sz, label_text) = if req_w > box_w - 6.0 {
                    let sz = (8.5 * (box_w - 6.0) / req_w).max(6.8);
                    (sz, p.label.clone())
                } else {
                    (8.5, p.label.clone())
                };
                let tw = crate::office::pdf::styled_string_width(&label_text, font_sz, true, false);
                let tx = (cx - tw / 2.0).max(bx + 2.0);
                page.add_styled_text(&label_text, tx, current_y - 13.5, font_sz, true, (0.2, 0.2, 0.2));
            }
        }
        current_y -= box_h + 10.0;

        // Hitung Y akhir untuk lifelines
        let lifeline_start_y = top_box_bottom_y;
        let lifeline_end_y = current_y - (diag.messages.len() as f32 * msg_h) - 4.0;

        // 3. Render Garis Hidup Vertikal (Lifeline)
        for p in &diag.participants {
            if let Some(&cx) = x_centers.get(&p.id) {
                Self::draw_dashed_vertical_line(pdf.current_page_mut(), cx, lifeline_start_y, lifeline_end_y, 0.8, (0.70, 0.75, 0.82));
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
                    page.add_styled_text(&msg.text, tx, arrow_y + 3.0, 8.0, false, (0.2, 0.22, 0.28));
                }

                // Garis Panah
                let is_dashed = msg.arrow == SequenceArrowType::DashedArrow || msg.arrow == SequenceArrowType::DashedLine;
                let color = (0.28, 0.30, 0.35);

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
                page.add_rounded_rect(bx, current_y - box_h, box_w, box_h, 4.0, true, (0.925, 0.925, 1.0));
                page.add_rounded_rect(bx, current_y - box_h, box_w, box_h, 4.0, false, (0.576, 0.439, 0.859));

                let req_w = crate::office::pdf::styled_string_width(&p.label, 8.5, true, false);
                let (font_sz, label_text) = if req_w > box_w - 6.0 {
                    let sz = (8.5 * (box_w - 6.0) / req_w).max(6.8);
                    (sz, p.label.clone())
                } else {
                    (8.5, p.label.clone())
                };
                let tw = crate::office::pdf::styled_string_width(&label_text, font_sz, true, false);
                let tx = (cx - tw / 2.0).max(bx + 2.0);
                page.add_styled_text(&label_text, tx, current_y - 13.5, font_sz, true, (0.2, 0.2, 0.2));
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

        let title_h = if diag.title.is_some() { 22.0 } else { 0.0 };
        let layer_h = 44.0;
        let total_diag_h = title_h + 12.0 + (diag.states.len() as f32 + 2.0) * layer_h;

        if current_y - total_diag_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        if let Some(ref t) = diag.title {
            let page = pdf.current_page_mut();
            let tw = crate::office::pdf::styled_string_width(t, 9.5, true, false);
            let tx = margin_x + (content_width - tw) / 2.0;
            page.add_styled_text(t, tx, current_y - 14.0, 9.5, true, (0.2, 0.22, 0.28));
            current_y -= title_h;
        }

        // Hitung posisi vertikal untuk tiap state
        let mid_x = margin_x + content_width / 2.0;
        let state_w = 140.0;
        let state_h = 24.0;

        let mut state_positions: HashMap<String, (f32, f32)> = HashMap::new();

        // 1. Initial State [*]
        let initial_y = current_y - 10.0;
        let page = pdf.current_page_mut();
        page.add_rounded_rect(mid_x - 6.0, initial_y - 6.0, 12.0, 12.0, 6.0, true, (0.2, 0.22, 0.28));
        state_positions.insert("[*]".to_string(), (mid_x, initial_y));
        current_y -= 36.0;

        // 2. States reguler
        for state_name in &diag.states {
            let sy = current_y - state_h / 2.0;
            let sx = mid_x - state_w / 2.0;
            let page = pdf.current_page_mut();

            // Kapsul state rounded
            page.add_rounded_rect(sx, current_y - state_h, state_w, state_h, 6.0, true, (0.925, 0.925, 1.0));
            page.add_rounded_rect(sx, current_y - state_h, state_w, state_h, 6.0, false, (0.576, 0.439, 0.859));

            let tw = crate::office::pdf::styled_string_width(state_name, 9.0, true, false);
            let tx = mid_x - tw / 2.0;
            page.add_styled_text(state_name, tx, current_y - 16.0, 9.0, true, (0.2, 0.2, 0.2));

            state_positions.insert(state_name.clone(), (mid_x, sy));
            current_y -= layer_h;
        }

        // 3. Final State [*]_end
        let final_y = current_y - 10.0;
        let page = pdf.current_page_mut();
        page.add_rounded_rect(mid_x - 8.0, final_y - 8.0, 16.0, 16.0, 8.0, false, (0.2, 0.22, 0.28));
        page.add_rounded_rect(mid_x - 4.5, final_y - 4.5, 9.0, 9.0, 4.5, true, (0.2, 0.22, 0.28));
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
                let arrow_color = (0.28, 0.30, 0.35);
                if y1 > y2 {
                    let start_pt = (x1, y1 - state_h / 2.0);
                    let end_pt = (x2, y2 + state_h / 2.0);
                    page.add_arrow(start_pt.0, start_pt.1, end_pt.0, end_pt.1, 1.2, arrow_color);

                    if let Some(ref lbl) = tr.label {
                        page.add_styled_text(lbl, (x1 + x2) / 2.0 + 8.0, (start_pt.1 + end_pt.1) / 2.0 - 3.0, 7.5, false, (0.35, 0.38, 0.45));
                    }
                } else {
                    let start_pt = (x1 + state_w / 2.0, y1);
                    let end_pt = (x2 + state_w / 2.0, y2);
                    page.add_arrow(start_pt.0, start_pt.1, end_pt.0, end_pt.1, 1.0, arrow_color);
                    if let Some(ref lbl) = tr.label {
                        page.add_styled_text(lbl, start_pt.0 + 6.0, (y1 + y2) / 2.0, 7.5, false, (0.35, 0.38, 0.45));
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

        let title_h = if diag.title.is_some() { 22.0 } else { 0.0 };
        let mut total_diag_h = title_h + 20.0;
        for c in &diag.classes {
            total_diag_h += 24.0 + (c.members.len() as f32 * 12.0) + 16.0;
        }

        if current_y - total_diag_h.min(300.0) < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        if let Some(ref t) = diag.title {
            let page = pdf.current_page_mut();
            let tw = crate::office::pdf::styled_string_width(t, 9.5, true, false);
            let tx = margin_x + (content_width - tw) / 2.0;
            page.add_styled_text(t, tx, current_y - 14.0, 9.5, true, (0.2, 0.22, 0.28));
            current_y -= title_h;
        }

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
            page.add_rounded_rect(bx, row_start_y - total_box_h, col_w, total_box_h, 4.0, true, (1.0, 1.0, 1.0));
            page.add_rounded_rect(bx, row_start_y - header_h, col_w, header_h, 4.0, true, (0.925, 0.925, 1.0));
            page.add_rounded_rect(bx, row_start_y - total_box_h, col_w, total_box_h, 4.0, false, (0.576, 0.439, 0.859));
            page.add_styled_line(bx, row_start_y - header_h, bx + col_w, row_start_y - header_h, 1.0, (0.576, 0.439, 0.859));

            let name_w = crate::office::pdf::styled_string_width(&cls.name, 9.0, true, false);
            let name_x = bx + (col_w - name_w) / 2.0;
            page.add_styled_text(&cls.name, name_x, row_start_y - 14.0, 9.0, true, (0.2, 0.2, 0.2));

            // 2. Body Kompartemen Anggota (Fields & Methods)
            let mut member_y = row_start_y - header_h - 11.0;
            for m in &cls.members {
                let color = if m.is_method { (0.10, 0.35, 0.55) } else { (0.20, 0.22, 0.26) };
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
                page.add_arrow(p1.0, p1.1, p2.0, p2.1, 1.2, (0.28, 0.30, 0.35));
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

    /// Render Diagram Lingkaran (Mermaid pie) Vektor Asli PDF
    fn render_pie_chart(
        pdf: &mut PdfDocument,
        diag: &WidyaPieChart,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
        page_height: f32,
    ) -> f32 {
        if diag.slices.is_empty() {
            return current_y;
        }

        let banner_h = 22.0;
        let chart_h = 160.0;
        let total_h = banner_h + chart_h + 20.0;

        if current_y - total_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        // Banner Judul
        let page = pdf.current_page_mut();
        let title_text = diag.title.as_ref()
            .map(|t| format!("DIAGRAM LINGKARAN: {}", t))
            .unwrap_or_else(|| "DIAGRAM LINGKARAN & DISTRIBUSI STATISTIK (PIE CHART)".to_string());
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, true, (0.92, 0.95, 0.99));
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, false, (0.18, 0.42, 0.76));
        page.add_styled_line(margin_x, current_y - banner_h, margin_x, current_y, 3.5, (0.1, 0.45, 0.9));
        page.add_styled_text(&title_text, margin_x + 12.0, current_y - 15.0, 9.5, true, (0.08, 0.22, 0.48));
        current_y -= banner_h + 14.0;

        // Wadah Vektor Diagram
        let page = pdf.current_page_mut();
        page.add_rect(margin_x, current_y - chart_h, content_width, chart_h, true, (0.985, 0.99, 1.0));
        page.add_rect(margin_x, current_y - chart_h, content_width, chart_h, false, (0.80, 0.85, 0.90));

        let total: f64 = diag.slices.iter().map(|s| s.value).sum();
        let r = 58.0f32;
        let cx = margin_x + 85.0;
        let cy = current_y - chart_h / 2.0;

        let colors: [(f32, f32, f32); 8] = [
            (0.12, 0.45, 0.85), // Biru Safir
            (0.12, 0.68, 0.42), // Hijau Zamrud
            (0.92, 0.58, 0.12), // Amber Emas
            (0.85, 0.25, 0.30), // Merah Koral
            (0.55, 0.30, 0.75), // Ungu Lembayung
            (0.10, 0.65, 0.75), // Biru Toska
            (0.85, 0.40, 0.65), // Merah Muda
            (0.40, 0.50, 0.60), // Slate
        ];

        let mut start_angle = 0.0f64;
        let num_slices = diag.slices.len();

        for (idx, slice) in diag.slices.iter().enumerate() {
            let sweep = if total > 0.0 {
                (slice.value / total) * 2.0 * std::f64::consts::PI
            } else {
                2.0 * std::f64::consts::PI / num_slices as f64
            };
            let end_angle = start_angle + sweep;
            let col = colors[idx % colors.len()];

            // Poligon irisan pie
            let steps = ((sweep / (std::f64::consts::PI / 18.0)).ceil() as usize).max(6);
            let mut pts = Vec::with_capacity(steps + 2);
            pts.push((cx, cy));
            for s in 0..=steps {
                let theta = start_angle + (s as f64 / steps as f64) * sweep;
                let px = cx + r * theta.cos() as f32;
                let py = cy + r * theta.sin() as f32;
                pts.push((px, py));
            }
            pts.push((cx, cy));

            page.add_polygon(pts, true, col);

            // Garis pembatas antar irisan
            page.add_styled_line(cx, cy, cx + r * start_angle.cos() as f32, cy + r * start_angle.sin() as f32, 1.2, (1.0, 1.0, 1.0));
            page.add_styled_line(cx, cy, cx + r * end_angle.cos() as f32, cy + r * end_angle.sin() as f32, 1.2, (1.0, 1.0, 1.0));

            // Label persentase jika irisan cukup lebar
            if sweep > 0.35 {
                let mid_theta = start_angle + sweep / 2.0;
                let text_r = r * 0.65;
                let tx = cx + text_r * mid_theta.cos() as f32;
                let ty = cy + text_r * mid_theta.sin() as f32;
                let pct = if total > 0.0 { (slice.value / total) * 100.0 } else { 0.0 };
                let pct_str = format!("{:.0}%", pct);
                let pw = crate::office::pdf::styled_string_width(&pct_str, 7.5, true, false);
                page.add_styled_text(&pct_str, tx - pw / 2.0, ty - 3.5, 7.5, true, (1.0, 1.0, 1.0));
            }

            start_angle = end_angle;
        }

        // Garis batas luar lingkaran
        let outer_steps = 48;
        let mut outer_pts = Vec::with_capacity(outer_steps + 1);
        for s in 0..=outer_steps {
            let theta = (s as f64 / outer_steps as f64) * 2.0 * std::f64::consts::PI;
            outer_pts.push((cx + r * theta.cos() as f32, cy + r * theta.sin() as f32));
        }
        page.add_polygon(outer_pts, false, (0.75, 0.80, 0.85));

        // Legenda di sisi kanan
        let legend_x = margin_x + 175.0;
        let legend_w = content_width - 185.0;
        let max_leg_items = diag.slices.len();
        let leg_row_h = (chart_h - 20.0) / max_leg_items.max(1) as f32;
        let mut leg_y = current_y - 20.0;

        for (idx, slice) in diag.slices.iter().enumerate() {
            let col = colors[idx % colors.len()];
            page.add_rect(legend_x, leg_y - 8.0, 12.0, 10.0, true, col);
            page.add_rect(legend_x, leg_y - 8.0, 12.0, 10.0, false, (0.4, 0.45, 0.5));

            let pct = if total > 0.0 { (slice.value / total) * 100.0 } else { 0.0 };
            let val_label = format!("{:.1}% ({:.1})", pct, slice.value);

            page.add_styled_text(&slice.label, legend_x + 18.0, leg_y - 6.0, 8.5, true, (0.15, 0.20, 0.30));
            let val_w = crate::office::pdf::styled_string_width(&val_label, 8.0, false, false);
            page.add_styled_text(&val_label, legend_x + legend_w - val_w, leg_y - 6.0, 8.0, false, (0.35, 0.40, 0.50));

            leg_y -= leg_row_h.min(18.0);
        }

        current_y -= chart_h + 14.0;
        current_y
    }

    /// Render Diagram Jadwal & Linimasa (Mermaid gantt) Vektor Asli PDF
    fn render_gantt_chart(
        pdf: &mut PdfDocument,
        diag: &WidyaGanttChart,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
        page_height: f32,
    ) -> f32 {
        if diag.tasks.is_empty() {
            return current_y;
        }

        let banner_h = 22.0;
        let header_row_h = 18.0;
        let task_row_h = 18.0;
        let chart_h = header_row_h + (diag.tasks.len() as f32 * task_row_h) + 14.0;
        let total_h = banner_h + chart_h + 20.0;

        if current_y - total_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        // Banner Judul
        let page = pdf.current_page_mut();
        let title_text = diag.title.as_ref()
            .map(|t| format!("DIAGRAM JADWAL & LINIMASA: {}", t))
            .unwrap_or_else(|| "DIAGRAM JADWAL & LINIMASA PROYEK (GANTT CHART)".to_string());
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, true, (0.92, 0.95, 0.99));
        page.add_rect(margin_x, current_y - banner_h, content_width, banner_h, false, (0.18, 0.42, 0.76));
        page.add_styled_line(margin_x, current_y - banner_h, margin_x, current_y, 3.5, (0.1, 0.45, 0.9));
        page.add_styled_text(&title_text, margin_x + 12.0, current_y - 15.0, 9.5, true, (0.08, 0.22, 0.48));
        current_y -= banner_h + 14.0;

        let mut max_time = 1.0f32;
        for t in &diag.tasks {
            let end_t = t.start + t.duration;
            if end_t > max_time {
                max_time = end_t;
            }
        }

        let label_col_w = 140.0f32;
        let timeline_w = content_width - label_col_w - 10.0;
        let timeline_x = margin_x + label_col_w;

        let page = pdf.current_page_mut();
        page.add_rect(margin_x, current_y - chart_h, content_width, chart_h, true, (0.985, 0.99, 1.0));
        page.add_rect(margin_x, current_y - chart_h, content_width, chart_h, false, (0.80, 0.85, 0.90));

        // Header Timeline (Grid Waktu)
        let header_y = current_y - header_row_h;
        page.add_styled_text("Tugas / Aktivitas", margin_x + 8.0, header_y + 4.0, 8.5, true, (0.12, 0.25, 0.45));
        page.add_styled_line(margin_x, header_y, margin_x + content_width, header_y, 1.0, (0.75, 0.80, 0.88));

        // Garis pemisah antara label kolom dan timeline
        page.add_styled_line(timeline_x, current_y, timeline_x, current_y - chart_h, 1.0, (0.80, 0.85, 0.90));

        // Sumbu interval waktu (5 pembagian)
        let num_ticks = 5;
        for i in 0..=num_ticks {
            let frac = i as f32 / num_ticks as f32;
            let tx = timeline_x + frac * timeline_w;
            let time_val = (frac * max_time).round();
            let label = format!("H+{}", time_val as i32);
            page.add_styled_text(&label, tx - 8.0, header_y + 4.0, 7.5, false, (0.35, 0.40, 0.50));
            page.add_styled_line(tx, header_y, tx, current_y - chart_h, 0.5, (0.88, 0.90, 0.94));
        }

        // Render setiap tugas
        let mut row_y = header_y - 14.0;
        for task in &diag.tasks {
            let bar_color = match task.status {
                GanttStatus::Done => (0.12, 0.65, 0.38),   // Hijau selesai
                GanttStatus::Active => (0.15, 0.45, 0.85), // Biru aktif
                GanttStatus::Crit => (0.85, 0.25, 0.20),   // Merah kritis
                GanttStatus::Normal => (0.40, 0.52, 0.65), // Abu netral
            };

            let task_w = crate::office::pdf::styled_string_width(&task.name, 8.0, false, false);
            let task_sz = if task_w > 115.0 {
                (8.0 * 115.0 / task_w).max(6.5)
            } else {
                8.0
            };
            page.add_styled_text(&task.name, margin_x + 8.0, row_y, task_sz, false, (0.15, 0.20, 0.28));

            let start_frac = (task.start / max_time).clamp(0.0, 1.0);
            let dur_frac = (task.duration / max_time).clamp(0.02, 1.0 - start_frac);
            let bx = timeline_x + start_frac * timeline_w;
            let bw = (dur_frac * timeline_w).max(8.0);
            let bh = 10.0f32;

            page.add_rect(bx, row_y - 2.0, bw, bh, true, bar_color);
            page.add_rect(bx, row_y - 2.0, bw, bh, false, (0.2, 0.3, 0.4));

            if bw > 24.0 {
                let badge = match task.status {
                    GanttStatus::Done => "[v]",
                    GanttStatus::Crit => "!",
                    _ => "",
                };
                if !badge.is_empty() {
                    page.add_styled_text(badge, bx + 3.0, row_y, 6.5, true, (1.0, 1.0, 1.0));
                }
            }

            row_y -= task_row_h;
        }

        current_y -= chart_h + 14.0;
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

    #[test]
    fn test_parse_and_render_pie_chart() {
        let code = r#"
        pie title Alokasi Sumber Daya Sistem
            "Kompiler ELF-64" : 40.0
            "Basis Data WidyaBase" : 30.0
            "Pustaka Kripto" : 20.0
            "Suite Dokumen" : 10.0
        "#;

        let pie = WidyaPieChart::parse(code);
        assert!(pie.is_some(), "Pie chart harus berhasil di-parse");
        let diag = pie.unwrap();
        assert_eq!(diag.title.as_deref(), Some("Alokasi Sumber Daya Sistem"));
        assert_eq!(diag.slices.len(), 4);
        assert_eq!(diag.slices[0].label, "Kompiler ELF-64");
        assert_eq!(diag.slices[0].value, 40.0);

        let mut pdf = PdfDocument::new("Uji Pie Chart");
        let final_y = WidyaDiagramEngine::render_to_pdf(&mut pdf, code, 50.0, 750.0, 480.0);
        assert!(final_y < 750.0);
        let bytes = pdf.generate_pdf_bytes();
        assert!(!bytes.is_empty());
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("DIAGRAM LINGKARAN"));
        assert!(text.contains("Kompiler ELF-64"));
        assert!(text.contains("40.0%"));
    }

    #[test]
    fn test_parse_and_render_gantt_chart() {
        let code = r#"
        gantt
            title Jadwal Rilis Fitur Baru
            dateFormat YYYY-MM-DD
            section Riset
                Spesifikasi Mesin :done, 0, 5
                Desain Vektor     :active, 5, 8
            section Eksekusi
                Implementasi Rust :crit, 8, 14
        "#;

        let gantt = WidyaGanttChart::parse(code);
        assert!(gantt.is_some(), "Gantt chart harus berhasil di-parse");
        let diag = gantt.unwrap();
        assert_eq!(diag.title.as_deref(), Some("Jadwal Rilis Fitur Baru"));
        assert_eq!(diag.tasks.len(), 3);
        assert_eq!(diag.tasks[0].name, "Spesifikasi Mesin");
        assert_eq!(diag.tasks[0].status, GanttStatus::Done);
        assert_eq!(diag.tasks[2].status, GanttStatus::Crit);

        let mut pdf = PdfDocument::new("Uji Gantt Chart");
        let final_y = WidyaDiagramEngine::render_to_pdf(&mut pdf, code, 50.0, 750.0, 480.0);
        assert!(final_y < 750.0);
        let bytes = pdf.generate_pdf_bytes();
        assert!(!bytes.is_empty());
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("DIAGRAM JADWAL"));
        assert!(text.contains("Spesifikasi Mesin"));
        assert!(text.contains("Implementasi Rust"));
    }
}
