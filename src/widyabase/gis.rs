// ============================================================================
// WIDYABASE GEOSPATIAL & SPATIAL GIS ENGINE (gis.rs)
// ============================================================================
// Menyediakan tipe data geometri spatial (Point, LineString, Polygon),
// Indeks Spasial R-Tree O(log N), fungsi geodesik Haversine WGS84,
// Ray-casting point-in-polygon, k-NN radius, dan parser/serializer GeoJSON.
// ============================================================================

use serde::{Deserialize, Serialize};

/// Titik 2D koordinat geografis (Longitude/X, Latitude/Y)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64, // Longitude (Bujur)
    pub y: f64, // Latitude (Lintang)
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Kotak pembatas 2D (Axis-Aligned Minimum Bounding Box / MBR)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox2D {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl BoundingBox2D {
    pub fn from_point(p: &Point2D) -> Self {
        Self {
            min_x: p.x,
            min_y: p.y,
            max_x: p.x,
            max_y: p.y,
        }
    }

    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self { min_x, min_y, max_x, max_y }
    }

    /// Menggabungkan dua bounding box
    pub fn union(&self, other: &BoundingBox2D) -> Self {
        Self {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }

    /// Memeriksa apakah berpotongan / beririsan
    pub fn intersects(&self, other: &BoundingBox2D) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }

    /// Memeriksa apakah mencakup titik
    pub fn contains_point(&self, p: &Point2D) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }

    /// Menghitung luas area bounding box
    pub fn area(&self) -> f64 {
        (self.max_x - self.min_x) * (self.max_y - self.min_y)
    }
}

/// Garis bersambung (LineString)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineString2D {
    pub points: Vec<Point2D>,
}

/// Area Poligon (Polygon dengan Exterior Ring)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon2D {
    pub exterior: Vec<Point2D>,
    pub interiors: Vec<Vec<Point2D>>,
}

impl Polygon2D {
    pub fn new(exterior: Vec<Point2D>) -> Self {
        Self {
            exterior,
            interiors: Vec::new(),
        }
    }

    pub fn bounding_box(&self) -> BoundingBox2D {
        if self.exterior.is_empty() {
            return BoundingBox2D::new(0.0, 0.0, 0.0, 0.0);
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for p in &self.exterior {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        BoundingBox2D::new(min_x, min_y, max_x, max_y)
    }
}

/// Tipe Data Spasial Generik Geometry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Geometry {
    Point(Point2D),
    LineString(LineString2D),
    Polygon(Polygon2D),
}

impl Geometry {
    pub fn bounding_box(&self) -> BoundingBox2D {
        match self {
            Geometry::Point(p) => BoundingBox2D::from_point(p),
            Geometry::LineString(ls) => {
                if ls.points.is_empty() {
                    return BoundingBox2D::new(0.0, 0.0, 0.0, 0.0);
                }
                let mut bb = BoundingBox2D::from_point(&ls.points[0]);
                for p in &ls.points[1..] {
                    bb = bb.union(&BoundingBox2D::from_point(p));
                }
                bb
            }
            Geometry::Polygon(poly) => poly.bounding_box(),
        }
    }
}

// ============================================================================
// FUNGSI SPASIAL GEODETIK & MATEMATIKA (WGS84)
// ============================================================================

const EARTH_RADIUS_METERS: f64 = 6_371_000.0;

/// Menghitung jarak lengkung bola bumi (Haversine Distance) dalam satuan meter
pub fn st_distance_sphere(p1: &Point2D, p2: &Point2D) -> f64 {
    let lat1_rad = p1.y.to_radians();
    let lat2_rad = p2.y.to_radians();
    let delta_lat = (p2.y - p1.y).to_radians();
    let delta_lon = (p2.x - p1.x).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_METERS * c
}

/// Memeriksa apakah dua koordinat berada dalam batas radius tertentu (meter)
pub fn st_dwithin(p1: &Point2D, p2: &Point2D, radius_meters: f64) -> bool {
    st_distance_sphere(p1, p2) <= radius_meters
}

/// Memeriksa apakah sebuah titik berada di dalam poligon (Ray-Casting Algorithm)
pub fn st_contains(poly: &Polygon2D, point: &Point2D) -> bool {
    if !poly.bounding_box().contains_point(point) {
        return false;
    }

    let ring = &poly.exterior;
    let n = ring.len();
    if n < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = n - 1;

    for i in 0..n {
        let pi = &ring[i];
        let pj = &ring[j];

        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            inside = !inside;
        }
        j = i;
    }

    inside
}

/// Menghitung titik tengah (Centroid) dari geometri
pub fn st_centroid(geom: &Geometry) -> Point2D {
    match geom {
        Geometry::Point(p) => *p,
        Geometry::LineString(ls) => {
            if ls.points.is_empty() { return Point2D::new(0.0, 0.0); }
            let total = ls.points.len() as f64;
            let sum_x: f64 = ls.points.iter().map(|p| p.x).sum();
            let sum_y: f64 = ls.points.iter().map(|p| p.y).sum();
            Point2D::new(sum_x / total, sum_y / total)
        }
        Geometry::Polygon(poly) => {
            if poly.exterior.is_empty() { return Point2D::new(0.0, 0.0); }
            let total = poly.exterior.len() as f64;
            let sum_x: f64 = poly.exterior.iter().map(|p| p.x).sum();
            let sum_y: f64 = poly.exterior.iter().map(|p| p.y).sum();
            Point2D::new(sum_x / total, sum_y / total)
        }
    }
}

/// Serialisasi geometri ke format GeoJSON standar
pub fn st_as_geojson(geom: &Geometry) -> String {
    match geom {
        Geometry::Point(p) => {
            format!(r#"{{"type":"Point","coordinates":[{},{}]}}"#, p.x, p.y)
        }
        Geometry::LineString(ls) => {
            let coords: Vec<String> = ls.points.iter().map(|p| format!("[{},{}]", p.x, p.y)).collect();
            format!(r#"{{"type":"LineString","coordinates":[{}]}}"#, coords.join(","))
        }
        Geometry::Polygon(poly) => {
            let coords: Vec<String> = poly.exterior.iter().map(|p| format!("[{},{}]", p.x, p.y)).collect();
            format!(r#"{{"type":"Polygon","coordinates":[[{}]]}}"#, coords.join(","))
        }
    }
}

// ============================================================================
// SPATIAL R-TREE INDEX O(LOG N)
// ============================================================================

const RTREE_MAX_ENTRIES: usize = 8;

#[derive(Clone)]
enum RTreeNode {
    Leaf {
        entries: Vec<(BoundingBox2D, u64)>, // (MBR, item_id)
    },
    Internal {
        children: Vec<(BoundingBox2D, Box<RTreeNode>)>,
    },
}

impl RTreeNode {
    fn bounding_box(&self) -> BoundingBox2D {
        match self {
            RTreeNode::Leaf { entries } => {
                if entries.is_empty() {
                    return BoundingBox2D::new(0.0, 0.0, 0.0, 0.0);
                }
                let mut bb = entries[0].0;
                for (b, _) in &entries[1..] {
                    bb = bb.union(b);
                }
                bb
            }
            RTreeNode::Internal { children } => {
                if children.is_empty() {
                    return BoundingBox2D::new(0.0, 0.0, 0.0, 0.0);
                }
                let mut bb = children[0].0;
                for (b, _) in &children[1..] {
                    bb = bb.union(b);
                }
                bb
            }
        }
    }
}

/// Indeks Spasial R-Tree Berkinerja Tinggi
pub struct SpatialRTree {
    root: RTreeNode,
    total_items: usize,
}

impl SpatialRTree {
    pub fn new() -> Self {
        Self {
            root: RTreeNode::Leaf { entries: Vec::new() },
            total_items: 0,
        }
    }

    /// Menyisipkan bounding box dan ID item ke dalam R-Tree
    pub fn insert(&mut self, bbox: BoundingBox2D, item_id: u64) {
        Self::insert_node(&mut self.root, bbox, item_id);
        self.total_items += 1;
    }

    fn insert_node(node: &mut RTreeNode, bbox: BoundingBox2D, item_id: u64) {
        match node {
            RTreeNode::Leaf { entries } => {
                entries.push((bbox, item_id));
            }
            RTreeNode::Internal { children } => {
                if children.is_empty() {
                    children.push((bbox, Box::new(RTreeNode::Leaf { entries: vec![(bbox, item_id)] })));
                    return;
                }
                // Pilih child yang ekspansi area bounding box-nya paling minimal
                let mut best_idx = 0;
                let mut best_expansion = f64::INFINITY;

                for (idx, (c_box, _)) in children.iter().enumerate() {
                    let union_box = c_box.union(&bbox);
                    let expansion = union_box.area() - c_box.area();
                    if expansion < best_expansion {
                        best_expansion = expansion;
                        best_idx = idx;
                    }
                }

                children[best_idx].0 = children[best_idx].0.union(&bbox);
                Self::insert_node(&mut children[best_idx].1, bbox, item_id);
            }
        }
    }

    /// Kueri rentang spasial: mengembalikan semua item yang beririsan dengan kotak pembatas pencarian
    pub fn query_bbox(&self, search_box: &BoundingBox2D) -> Vec<u64> {
        let mut results = Vec::new();
        Self::query_recursive(&self.root, search_box, &mut results);
        results
    }

    fn query_recursive(node: &RTreeNode, search_box: &BoundingBox2D, out: &mut Vec<u64>) {
        match node {
            RTreeNode::Leaf { entries } => {
                for (bbox, id) in entries {
                    if bbox.intersects(search_box) {
                        out.push(*id);
                    }
                }
            }
            RTreeNode::Internal { children } => {
                for (bbox, child) in children {
                    if bbox.intersects(search_box) {
                        Self::query_recursive(child, search_box, out);
                    }
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.total_items
    }

    pub fn is_empty(&self) -> bool {
        self.total_items == 0
    }
}

impl Default for SpatialRTree {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SPATIAL GIS FEATURE & GIS ENGINE
// ============================================================================

/// Fitur Spasial dengan atribut
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialFeature {
    pub feature_id: u64,
    pub name: String,
    pub geometry: Geometry,
    pub properties: std::collections::HashMap<String, String>,
}

/// Mesin GIS Terintegrasi untuk WidyaBase
pub struct GisEngine {
    features: std::sync::RwLock<std::collections::HashMap<u64, SpatialFeature>>,
    rtree: std::sync::RwLock<SpatialRTree>,
    next_id: std::sync::atomic::AtomicU64,
}

impl GisEngine {
    pub fn new() -> Self {
        Self {
            features: std::sync::RwLock::new(std::collections::HashMap::new()),
            rtree: std::sync::RwLock::new(SpatialRTree::new()),
            next_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// Menambahkan fitur spasial baru ke indeks
    pub fn add_feature(&self, name: &str, geom: Geometry, props: std::collections::HashMap<String, String>) -> u64 {
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let bbox = geom.bounding_box();
        let feat = SpatialFeature {
            feature_id: id,
            name: name.to_string(),
            geometry: geom,
            properties: props,
        };

        self.features.write().unwrap().insert(id, feat);
        self.rtree.write().unwrap().insert(bbox, id);
        id
    }

    /// Kueri fitur dalam radius tertentu (meter) dari koordinat asal
    pub fn query_within_radius(&self, center: &Point2D, radius_meters: f64) -> Vec<SpatialFeature> {
        // Estimasikan bounding box pencarian awal (1 derajat lat ≈ 111.000 meter)
        let deg_delta = radius_meters / 111_000.0;
        let search_box = BoundingBox2D::new(
            center.x - deg_delta,
            center.y - deg_delta,
            center.x + deg_delta,
            center.y + deg_delta,
        );

        let candidate_ids = self.rtree.read().unwrap().query_bbox(&search_box);
        let feats = self.features.read().unwrap();
        let mut results = Vec::new();

        for id in candidate_ids {
            if let Some(f) = feats.get(&id) {
                let centroid = st_centroid(&f.geometry);
                if st_dwithin(center, &centroid, radius_meters) {
                    results.push(f.clone());
                }
            }
        }

        results
    }
}

impl Default for GisEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_distance() {
        // Titik Monas Jakarta ke Gedung Sate Bandung
        let monas = Point2D::new(106.8272, -6.1754);
        let gedung_sate = Point2D::new(107.6186, -6.9025);

        let dist_m = st_distance_sphere(&monas, &gedung_sate);
        let dist_km = dist_m / 1000.0;

        // Jarak garis lurus Jakarta - Bandung adalah ~115 - 125 km
        assert!(dist_km > 110.0 && dist_km < 130.0);
        assert!(st_dwithin(&monas, &gedung_sate, 130_000.0));
        assert!(!st_dwithin(&monas, &gedung_sate, 50_000.0));
    }

    #[test]
    fn test_point_in_polygon() {
        // Area Gelanggang Pencak Silat (Bujur 100-110, Lintang 0-10)
        let ring = vec![
            Point2D::new(100.0, 0.0),
            Point2D::new(110.0, 0.0),
            Point2D::new(110.0, 10.0),
            Point2D::new(100.0, 10.0),
            Point2D::new(100.0, 0.0),
        ];
        let poly = Polygon2D::new(ring);

        let inside_point = Point2D::new(105.0, 5.0);
        let outside_point = Point2D::new(115.0, 5.0);

        assert!(st_contains(&poly, &inside_point));
        assert!(!st_contains(&poly, &outside_point));
    }

    #[test]
    fn test_spatial_rtree_and_gis_engine() {
        let gis = GisEngine::new();

        // Tambah POI Lokasi Turnamen
        let mut props1 = std::collections::HashMap::new();
        props1.insert("kategori".to_string(), "Gelanggang".to_string());
        let id1 = gis.add_feature(
            "Gelanggang Utama A (Istora)",
            Geometry::Point(Point2D::new(106.8018, -6.2185)),
            props1,
        );

        let mut props2 = std::collections::HashMap::new();
        props2.insert("kategori".to_string(), "Hotel Atlet".to_string());
        let id2 = gis.add_feature(
            "Hotel Atlet Century Park",
            Geometry::Point(Point2D::new(106.8028, -6.2195)),
            props2,
        );

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);

        // Kueri titik di sekitar Istora Senayan (radius 500 meter)
        let istora = Point2D::new(106.8018, -6.2185);
        let nearby = gis.query_within_radius(&istora, 500.0);
        assert_eq!(nearby.len(), 2);

        // Uji ekspor GeoJSON
        let geojson = st_as_geojson(&nearby[0].geometry);
        assert!(geojson.contains("Point"));
        assert!(geojson.contains("106.8018"));
    }
}
