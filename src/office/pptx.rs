use crate::office::zip::ZipPackage;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Tema Visual Slide
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SlideTheme {
    DarkModern,
    ExecutiveNavy,
    EmeraldFintech,
    SovereignGold,
}

impl Default for SlideTheme {
    fn default() -> Self {
        SlideTheme::DarkModern
    }
}

/// Tata Letak Slide Presentasi
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SlideLayout {
    TitleSlide,
    TitleAndContent,
    TwoColumns,
    KpiDashboard,
    SectionHeader,
    Blank,
}

/// Kartu Metrik KPI untuk Slide Dashboard Eksekutif
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricKpiCard {
    pub label: String,
    pub value: String,
    pub subtext: String,
    pub is_positive: bool,
}

/// Slide Presentasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PptxSlide {
    pub layout: SlideLayout,
    pub title: String,
    pub subtitle: Option<String>,
    pub bullets: Vec<String>,
    pub right_bullets: Option<Vec<String>>,
    pub kpi_metrics: Option<Vec<MetricKpiCard>>,
    pub notes: Option<String>,
}

impl PptxSlide {
    pub fn title_slide(title: &str, subtitle: &str) -> Self {
        Self {
            layout: SlideLayout::TitleSlide,
            title: title.to_string(),
            subtitle: Some(subtitle.to_string()),
            bullets: Vec::new(),
            right_bullets: None,
            kpi_metrics: None,
            notes: None,
        }
    }

    pub fn content_slide(title: &str, bullets: &[&str]) -> Self {
        Self {
            layout: SlideLayout::TitleAndContent,
            title: title.to_string(),
            subtitle: None,
            bullets: bullets.iter().map(|s| s.to_string()).collect(),
            right_bullets: None,
            kpi_metrics: None,
            notes: None,
        }
    }

    pub fn two_column_slide(title: &str, left_bullets: &[&str], right_bullets: &[&str]) -> Self {
        Self {
            layout: SlideLayout::TwoColumns,
            title: title.to_string(),
            subtitle: None,
            bullets: left_bullets.iter().map(|s| s.to_string()).collect(),
            right_bullets: Some(right_bullets.iter().map(|s| s.to_string()).collect()),
            kpi_metrics: None,
            notes: None,
        }
    }

    pub fn kpi_slide(title: &str, metrics: Vec<MetricKpiCard>) -> Self {
        Self {
            layout: SlideLayout::KpiDashboard,
            title: title.to_string(),
            subtitle: None,
            bullets: Vec::new(),
            right_bullets: None,
            kpi_metrics: Some(metrics),
            notes: None,
        }
    }

    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_string());
        self
    }
}

/// Dokumen Presentasi PowerPoint (.pptx / OpenXML Presentation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PptxPresentation {
    pub title: String,
    pub author: String,
    pub theme: SlideTheme,
    pub slides: Vec<PptxSlide>,
}

impl PptxPresentation {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            author: "Widya Office Suite".to_string(),
            theme: SlideTheme::DarkModern,
            slides: Vec::new(),
        }
    }

    pub fn set_theme(&mut self, theme: SlideTheme) {
        self.theme = theme;
    }

    pub fn add_slide(&mut self, slide: PptxSlide) {
        self.slides.push(slide);
    }

    /// Menghasilkan Berkas Biner .PPTX Asli (Zip Packaging Berstandar ECMA-376)
    pub fn export_to_pptx_bytes(&self) -> Vec<u8> {
        let mut pkg = ZipPackage::new();

        let num_slides = self.slides.len().max(1);

        // 1. [Content_Types].xml
        let mut content_types = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
  <Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
  <Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
  <Override PartName="/ppt/tableStyles.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"/>
  <Override PartName="/ppt/presProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"/>
  <Override PartName="/ppt/viewProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
"#);
        for i in 1..=num_slides {
            content_types.push_str(&format!(
                r#"  <Override PartName="/ppt/slides/slide{}.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
"#,
                i
            ));
        }
        content_types.push_str("</Types>");
        pkg.add_file_str("[Content_Types].xml", &content_types);

        // 2. _rels/.rels
        pkg.add_file_str(
            "_rels/.rels",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#,
        );

        // 3. ppt/presentation.xml
        let mut sld_id_lst = String::new();
        for i in 1..=num_slides {
            sld_id_lst.push_str(&format!(r#"    <p:sldId id="{}" r:id="rId{}"/>
"#, 255 + i, i));
        }

        let pres_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:sldMasterIdLst>
    <p:sldMasterId id="2147483648" r:id="rIdMaster1"/>
  </p:sldMasterIdLst>
  <p:sldIdLst>
{}  </p:sldIdLst>
  <p:sldSz cx="12192000" cy="6858000" type="screen16x9"/>
  <p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#,
            sld_id_lst
        );
        pkg.add_file_str("ppt/presentation.xml", &pres_xml);

        // 4. ppt/_rels/presentation.xml.rels
        let mut pres_rels = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rIdMaster1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
  <Relationship Id="rIdPresProps" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps" Target="presProps.xml"/>
  <Relationship Id="rIdViewProps" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps" Target="viewProps.xml"/>
  <Relationship Id="rIdTableStyles" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles" Target="tableStyles.xml"/>
"#);
        for i in 1..=num_slides {
            pres_rels.push_str(&format!(
                r#"  <Relationship Id="rId{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide{}.xml"/>
"#,
                i, i
            ));
        }
        pres_rels.push_str("</Relationships>");
        pkg.add_file_str("ppt/_rels/presentation.xml.rels", &pres_rels);

        // 5. Slide Master, Layout, Theme, PresProps, ViewProps, TableStyles
        let (bg_color, accent_color, card_color) = match self.theme {
            SlideTheme::DarkModern => ("0F172A", "38BDF8", "1E293B"),
            SlideTheme::ExecutiveNavy => ("030712", "60A5FA", "0F172A"),
            SlideTheme::EmeraldFintech => ("022C22", "34D399", "064E3B"),
            SlideTheme::SovereignGold => ("1C1917", "FBBF24", "292524"),
        };

        // Theme 1
        let theme_xml = format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Office Theme">
  <a:themeElements>
    <a:clrScheme name="Office">
      <a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
      <a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="{}"/></a:dk2>
      <a:lt2><a:srgbClr val="F8FAFC"/></a:lt2>
      <a:accent1><a:srgbClr val="{}"/></a:accent1>
      <a:accent2><a:srgbClr val="34D399"/></a:accent2>
      <a:accent3><a:srgbClr val="FBBF24"/></a:accent3>
      <a:accent4><a:srgbClr val="F87171"/></a:accent4>
      <a:accent5><a:srgbClr val="A855F7"/></a:accent5>
      <a:accent6><a:srgbClr val="64748B"/></a:accent6>
      <a:hlink><a:srgbClr val="{}"/></a:hlink>
      <a:folHlink><a:srgbClr val="818CF8"/></a:folHlink>
    </a:clrScheme>
    <a:fontScheme name="Office">
      <a:majorFont><a:latin typeface="Segoe UI"/><a:ea typeface=""/><a:cs typeface=""/></a:majorFont>
      <a:minorFont><a:latin typeface="Segoe UI"/><a:ea typeface=""/><a:cs typeface=""/></a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="Office">
      <a:fillStyleLst>
        <a:solidFill><a:srgbClr val="{}"/></a:solidFill>
        <a:solidFill><a:srgbClr val="{}"/></a:solidFill>
        <a:solidFill><a:srgbClr val="334155"/></a:solidFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="9525"><a:solidFill><a:srgbClr val="334155"/></a:solidFill></a:ln>
        <a:ln w="25400"><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:ln>
        <a:ln w="38100"><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectStyle><a:effectLst/></a:effectStyle>
        <a:effectStyle><a:effectLst/></a:effectStyle>
        <a:effectStyle><a:effectLst/></a:effectStyle>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill><a:srgbClr val="{}"/></a:solidFill>
        <a:solidFill><a:srgbClr val="{}"/></a:solidFill>
        <a:solidFill><a:srgbClr val="020617"/></a:solidFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
  <a:objectDefaults/>
  <a:extraClrSchemeLst/>
</a:theme>"#, card_color, accent_color, accent_color, bg_color, card_color, accent_color, accent_color, bg_color, card_color);
        pkg.add_file_str("ppt/theme/theme1.xml", &theme_xml);

        // Slide Master 1
        pkg.add_file_str(
            "ppt/slideMasters/slideMaster1.xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
  <p:sldLayoutIdLst>
    <p:sldLayoutId id="2147483649" r:id="rId1"/>
  </p:sldLayoutIdLst>
</p:sldMaster>"#,
        );

        pkg.add_file_str(
            "ppt/slideMasters/_rels/slideMaster1.xml.rels",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>
</Relationships>"#,
        );

        // Slide Layout 1
        pkg.add_file_str(
            "ppt/slideLayouts/slideLayout1.xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" type="blank" preserve="1">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>
</p:sldLayout>"#,
        );

        pkg.add_file_str(
            "ppt/slideLayouts/_rels/slideLayout1.xml.rels",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
</Relationships>"#,
        );

        // TableStyles, PresProps, ViewProps
        pkg.add_file_str(
            "ppt/tableStyles.xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:tblStyleLst xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" def="{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}"/>"#,
        );

        pkg.add_file_str(
            "ppt/presProps.xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentationPr xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>"#,
        );

        pkg.add_file_str(
            "ppt/viewProps.xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:viewPr xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:normalViewPr><p:restoredLeft sz="15621"/><p:restoredTop sz="94660"/></p:normalViewPr>
  <p:slideViewPr><p:cSldViewPr><p:cViewPr varScale="1"><p:scale><a:sx n="100" d="100"/><a:sy n="100" d="100"/></p:scale><p:origin x="0" y="0"/></p:cViewPr></p:cSldViewPr></p:slideViewPr>
</p:viewPr>"#,
        );

        // 6. Generate Every Slide (slide1.xml, slide2.xml, ...)

        for (idx, slide) in self.slides.iter().enumerate() {
            let slide_num = idx + 1;
            let mut shapes_xml = String::new();
            let mut sp_id = 2;

            // Shape: Background Canvas
            shapes_xml.push_str(&format!(
                r#"      <p:sp>
        <p:nvSpPr><p:cNvPr id="{}" name="Background"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="12192000" cy="6858000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="{}"/></a:solidFill>
        </p:spPr>
      </p:sp>
"#,
                sp_id, bg_color
            ));
            sp_id += 1;

            // Shape: Top Accent Bar
            shapes_xml.push_str(&format!(
                r#"      <p:sp>
        <p:nvSpPr><p:cNvPr id="{}" name="TopBar"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="800000" y="450000"/><a:ext cx="10592000" cy="60000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="{}"/></a:solidFill>
        </p:spPr>
      </p:sp>
"#,
                sp_id, accent_color
            ));
            sp_id += 1;

            // Render Content based on Slide Layout
            match slide.layout {
                SlideLayout::TitleSlide => {
                    // Main Title
                    let safe_title = escape_xml(&slide.title);
                    shapes_xml.push_str(&format!(
                        r#"      <p:sp>
        <p:nvSpPr><p:cNvPr id="{}" name="Title"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="800000" y="1600000"/><a:ext cx="10592000" cy="2200000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr"/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="id-ID" sz="3600" b="1"><a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill></a:rPr>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
                        sp_id, safe_title
                    ));
                    sp_id += 1;

                    // Subtitle
                    if let Some(ref sub) = slide.subtitle {
                        let safe_sub = escape_xml(sub);
                        shapes_xml.push_str(&format!(
                            r#"      <p:sp>
        <p:nvSpPr><p:cNvPr id="{}" name="Subtitle"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="800000" y="4000000"/><a:ext cx="10592000" cy="1200000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="t"/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="id-ID" sz="2000"><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:rPr>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
                            sp_id, accent_color, safe_sub
                        ));
                    }
                }
                SlideLayout::KpiDashboard => {
                    // Header Title
                    let safe_title = escape_xml(&slide.title);
                    shapes_xml.push_str(&format!(
                        r#"      <p:sp>
        <p:nvSpPr><p:cNvPr id="{}" name="Header"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="800000" y="650000"/><a:ext cx="10592000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="id-ID" sz="2800" b="1"><a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill></a:rPr>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
                        sp_id, safe_title
                    ));
                    sp_id += 1;

                    // Metric Cards
                    if let Some(ref cards) = slide.kpi_metrics {
                        let card_w = 3200000i64;
                        let card_h = 4200000i64;
                        let gap = 496000i64;
                        let start_x = 800000i64;
                        let start_y = 1700000i64;

                        for (c_i, card) in cards.iter().take(3).enumerate() {
                            let cx = start_x + (c_i as i64) * (card_w + gap);

                            // Card Background
                            shapes_xml.push_str(&format!(
                                r#"      <p:sp>
        <p:nvSpPr><p:cNvPr id="{}" name="CardBg{}"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></a:xfrm>
          <a:prstGeom prst="roundRect"><a:avLst><a:gd name="adj" fmla="val 5000"/></a:avLst></a:prstGeom>
          <a:solidFill><a:srgbClr val="{}"/></a:solidFill>
          <a:ln w="25400"><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr" lIns="200000" rIns="200000" tIns="200000" bIns="200000"/>
          <a:lstStyle/>
          <a:p>
            <a:pPr algn="ctr"/>
            <a:r>
              <a:rPr lang="id-ID" sz="1600" b="1"><a:solidFill><a:srgbClr val="94A3B8"/></a:solidFill></a:rPr>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
          <a:p>
            <a:pPr algn="ctr"/>
            <a:r>
              <a:rPr lang="id-ID" sz="3800" b="1"><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:rPr>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
          <a:p>
            <a:pPr algn="ctr"/>
            <a:r>
              <a:rPr lang="id-ID" sz="1300"><a:solidFill><a:srgbClr val="E2E8F0"/></a:solidFill></a:rPr>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
                                sp_id, c_i, cx, start_y, card_w, card_h, card_color, accent_color,
                                escape_xml(&card.label),
                                accent_color,
                                escape_xml(&card.value),
                                escape_xml(&card.subtext)
                            ));
                            sp_id += 1;
                        }
                    }
                }
                _ => {
                    // Default Content / Bullet Slide
                    let safe_title = escape_xml(&slide.title);
                    shapes_xml.push_str(&format!(
                        r#"      <p:sp>
        <p:nvSpPr><p:cNvPr id="{}" name="Header"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="800000" y="650000"/><a:ext cx="10592000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="id-ID" sz="2800" b="1"><a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill></a:rPr>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
                        sp_id, safe_title
                    ));
                    sp_id += 1;

                    // Bullets Box
                    let mut bullets_xml = String::new();
                    for bullet in &slide.bullets {
                        bullets_xml.push_str(&format!(
                            r#"          <a:p>
            <a:pPr marL="280000" indent="-280000">
              <a:buChar char="•"/>
            </a:pPr>
            <a:r>
              <a:rPr lang="id-ID" sz="2000"><a:solidFill><a:srgbClr val="E2E8F0"/></a:solidFill></a:rPr>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
"#,
                            escape_xml(bullet)
                        ));
                    }

                    shapes_xml.push_str(&format!(
                        r#"      <p:sp>
        <p:nvSpPr><p:cNvPr id="{}" name="Content"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="800000" y="1700000"/><a:ext cx="10592000" cy="4500000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="t"/>
          <a:lstStyle/>
{}        </p:txBody>
      </p:sp>
"#,
                        sp_id, bullets_xml
                    ));
                }
            }

            // Slide XML
            let slide_xml = format!(
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
{}    </p:spTree>
  </p:cSld>
  <p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>
</p:sld>"#,
                shapes_xml
            );
            pkg.add_file_str(&format!("ppt/slides/slide{}.xml", slide_num), &slide_xml);

            // Slide rels
            let slide_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
</Relationships>"#;
            pkg.add_file_str(&format!("ppt/slides/_rels/slide{}.xml.rels", slide_num), slide_rels);
        }

        // 7. docProps/core.xml & app.xml
        let now = chrono::Utc::now().to_rfc3339();
        let core_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title>{}</dc:title>
  <dc:creator>{}</dc:creator>
  <dcterms:created xsi:type="dcterms:W3CDTF">{}</dcterms:created>
</cp:coreProperties>"#,
            escape_xml(&self.title), escape_xml(&self.author), now
        );
        pkg.add_file_str("docProps/core.xml", &core_xml);

        let app_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <TotalTime>0</TotalTime>
  <Slides>{}</Slides>
  <Company>Widya-Lang Sovereign Enterprise</Company>
</Properties>"#,
            num_slides
        );
        pkg.add_file_str("docProps/app.xml", &app_xml);

        pkg.to_bytes()
    }

    /// Simpan Langsung ke Berkas Fisik `.pptx`
    pub fn save_as_pptx<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let bytes = self.export_to_pptx_bytes();
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    /// Render ke Web Slide Viewer HTML5 Interaktif Standalone (Keynote / PowerPoint Web)
    pub fn render_html_slideshow(&self) -> String {
        let (bg_gradient, accent_color, card_bg) = match self.theme {
            SlideTheme::DarkModern => (
                "linear-gradient(135deg, #0f172a 0%, #1e293b 100%)",
                "#38bdf8",
                "rgba(30, 41, 59, 0.7)",
            ),
            SlideTheme::ExecutiveNavy => (
                "linear-gradient(135deg, #030712 0%, #0f172a 100%)",
                "#60a5fa",
                "rgba(15, 23, 42, 0.8)",
            ),
            SlideTheme::EmeraldFintech => (
                "linear-gradient(135deg, #064e3b 0%, #022c22 100%)",
                "#34d399",
                "rgba(6, 78, 59, 0.7)",
            ),
            SlideTheme::SovereignGold => (
                "linear-gradient(135deg, #1c1917 0%, #292524 100%)",
                "#fbbf24",
                "rgba(41, 37, 36, 0.8)",
            ),
        };

        let mut html = String::from("<!DOCTYPE html><html><head><meta charset='utf-8'>");
        html.push_str(&format!("<title>{} - Presentasi WidyaOffice</title>", self.title));
        html.push_str(&format!(r#"<style>
            * {{ box-sizing: border-box; }}
            body {{ margin: 0; background: #020617; color: white; font-family: 'Segoe UI', -apple-system, sans-serif; overflow: hidden; display: flex; flex-direction: column; height: 100vh; }}
            .slide-viewport {{ flex: 1; display: flex; justify-content: center; align-items: center; padding: 20px; }}
            .slide-canvas {{ width: 1280px; height: 720px; background: {}; border-radius: 16px; box-shadow: 0 25px 50px -12px rgba(0,0,0,0.6); padding: 60px 80px; display: none; flex-direction: column; justify-content: center; position: relative; border: 1px solid #334155; }}
            .slide-canvas.active {{ display: flex; animation: fadeIn 0.3s ease; }}
            @keyframes fadeIn {{ from {{ opacity: 0; transform: scale(0.98); }} to {{ opacity: 1; transform: scale(1); }} }}
            h1 {{ font-size: 3.5rem; color: {}; margin: 0 0 16px 0; line-height: 1.2; font-weight: 800; }}
            h2 {{ font-size: 2.2rem; color: #f8fafc; margin: 0 0 24px 0; border-bottom: 2px solid {}; padding-bottom: 12px; }}
            p.subtitle {{ font-size: 1.8rem; color: #94a3b8; line-height: 1.4; margin: 0; }}
            ul {{ font-size: 1.6rem; line-height: 1.8; color: #e2e8f0; margin: 0; padding-left: 30px; }}
            .two-col {{ display: grid; grid-template-columns: 1fr 1fr; gap: 40px; }}
            .kpi-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 24px; margin-top: 20px; }}
            .kpi-card {{ background: {}; border: 1px solid #334155; border-radius: 12px; padding: 24px; }}
            .kpi-label {{ font-size: 1.1rem; color: #94a3b8; text-transform: uppercase; letter-spacing: 1px; margin-bottom: 8px; }}
            .kpi-val {{ font-size: 2.5rem; font-weight: 800; color: #f8fafc; margin-bottom: 8px; }}
            .kpi-sub {{ font-size: 1.0rem; color: #34d399; }}
            .controls {{ height: 60px; background: #020617; border-top: 1px solid #1e293b; display: flex; justify-content: space-between; align-items: center; padding: 0 30px; }}
            .nav-btn {{ background: {}; color: #020617; border: none; padding: 8px 18px; font-size: 14px; font-weight: bold; border-radius: 6px; cursor: pointer; }}
            .nav-btn:hover {{ opacity: 0.9; }}
            .slide-counter {{ font-size: 14px; color: #94a3b8; font-weight: 500; }}
        </style></head><body>"#, bg_gradient, accent_color, accent_color, card_bg, accent_color));

        html.push_str("<div class='slide-viewport'>");

        for (i, slide) in self.slides.iter().enumerate() {
            let active_cls = if i == 0 { " active" } else { "" };
            html.push_str(&format!("<div class='slide-canvas{}' id='slide-{}'>", active_cls, i + 1));

            match slide.layout {
                SlideLayout::TitleSlide => {
                    html.push_str(&format!("<h1>{}</h1>", slide.title));
                    if let Some(ref sub) = slide.subtitle {
                        html.push_str(&format!("<p class='subtitle'>{}</p>", sub));
                    }
                }
                SlideLayout::TitleAndContent => {
                    html.push_str(&format!("<h2>{}</h2>", slide.title));
                    html.push_str("<ul>");
                    for b in &slide.bullets {
                        html.push_str(&format!("<li>{}</li>", b));
                    }
                    html.push_str("</ul>");
                }
                SlideLayout::TwoColumns => {
                    html.push_str(&format!("<h2>{}</h2>", slide.title));
                    html.push_str("<div class='two-col'>");
                    html.push_str("<ul>");
                    for b in &slide.bullets {
                        html.push_str(&format!("<li>{}</li>", b));
                    }
                    html.push_str("</ul>");
                    if let Some(ref rb) = slide.right_bullets {
                        html.push_str("<ul>");
                        for b in rb {
                            html.push_str(&format!("<li>{}</li>", b));
                        }
                        html.push_str("</ul>");
                    }
                    html.push_str("</div>");
                }
                SlideLayout::KpiDashboard => {
                    html.push_str(&format!("<h2>{}</h2>", slide.title));
                    if let Some(ref metrics) = slide.kpi_metrics {
                        html.push_str("<div class='kpi-grid'>");
                        for m in metrics {
                            let color = if m.is_positive { "#34d399" } else { "#f87171" };
                            html.push_str(&format!(
                                "<div class='kpi-card'><div class='kpi-label'>{}</div><div class='kpi-val'>{}</div><div class='kpi-sub' style='color:{};'>{}</div></div>",
                                m.label, m.value, color, m.subtext
                            ));
                        }
                        html.push_str("</div>");
                    }
                }
                _ => {
                    html.push_str(&format!("<h2>{}</h2>", slide.title));
                }
            }

            html.push_str("</div>");
        }

        html.push_str("</div>");

        // Controls bar & JS navigation
        html.push_str(&format!(r#"<div class='controls'>
            <button class='nav-btn' onclick='prevSlide()'>◀ Sebelumnya</button>
            <div class='slide-counter'><span id='curr-idx'>1</span> / {}</div>
            <button class='nav-btn' onclick='nextSlide()'>Berikutnya ▶</button>
        </div>"#, self.slides.len()));

        html.push_str(&format!(r#"<script>
            let current = 1;
            const total = {};
            function showSlide(idx) {{
                if (idx < 1 || idx > total) return;
                document.querySelectorAll('.slide-canvas').forEach(s => s.classList.remove('active'));
                document.getElementById('slide-' + idx).classList.add('active');
                document.getElementById('curr-idx').innerText = idx;
                current = idx;
            }}
            function nextSlide() {{ showSlide(current + 1); }}
            function prevSlide() {{ showSlide(current - 1); }}
            window.addEventListener('keydown', (e) => {{
                if (e.key === 'ArrowRight' || e.key === ' ') nextSlide();
                if (e.key === 'ArrowLeft') prevSlide();
            }});
        </script></body></html>"#, self.slides.len()));

        html
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
