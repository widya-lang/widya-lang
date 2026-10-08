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

        let diagram = WidyaDiagram::parse(source);

        // 2. Jika diagram memiliki Subgraph (Diagram Arsitektur Sistem)
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
}
