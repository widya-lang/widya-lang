use crate::adengine::types::{AdCreative, AdFormat};

/// Generator Standar IAB VAST 4.2 (Digital Video Ad Serving Template) & VMAP
pub struct VastEngine;

impl VastEngine {
    /// Hasilkan XML VAST 4.2 Resmi untuk Pemutar Video HTML5 / WebGL / Mobile
    pub fn generate_vast_xml(creative: &AdCreative) -> String {
        let (duration_str, skip_offset_str) = match &creative.format {
            AdFormat::LinearPreRoll { skip_offset_sec, duration_sec } => {
                (format!("00:00:{:02}", duration_sec), format!("skipoffset=\"00:00:{:02}\"", skip_offset_sec))
            }
            AdFormat::LinearMidRoll { skip_offset_sec, duration_sec, .. } => {
                (format!("00:00:{:02}", duration_sec), format!("skipoffset=\"00:00:{:02}\"", skip_offset_sec))
            }
            AdFormat::LinearPostRoll { duration_sec } => {
                (format!("00:00:{:02}", duration_sec), String::new())
            }
            _ => ("00:00:15".to_string(), "skipoffset=\"00:00:05\"".to_string()),
        };

        format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<VAST version="4.2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <Ad id="{ad_id}">
    <InLine>
      <AdSystem version="2.0">WidyaAd Sovereign Engine (ISO 32000-1 &amp; UU PDP)</AdSystem>
      <AdTitle>{title}</AdTitle>
      <Description>{desc}</Description>
      <Advertiser>{advertiser}</Advertiser>
      <Pricing model="CPM" currency="IDR">25000</Pricing>
      <Error><![CDATA[https://ad.widya.id/api/telemetry/error?id={ad_id}]]></Error>
      <Impression id="WIDYA_IMP_{ad_id}"><![CDATA[https://ad.widya.id/api/telemetry/impression?id={ad_id}]]></Impression>
      <Creatives>
        <Creative id="CR_{ad_id}" sequence="1">
          <Linear {skip_offset_str}>
            <Duration>{duration_str}</Duration>
            <TrackingEvents>
              <Tracking event="start"><![CDATA[https://ad.widya.id/api/telemetry/start?id={ad_id}]]></Tracking>
              <Tracking event="firstQuartile"><![CDATA[https://ad.widya.id/api/telemetry/firstQuartile?id={ad_id}]]></Tracking>
              <Tracking event="midpoint"><![CDATA[https://ad.widya.id/api/telemetry/midpoint?id={ad_id}]]></Tracking>
              <Tracking event="thirdQuartile"><![CDATA[https://ad.widya.id/api/telemetry/thirdQuartile?id={ad_id}]]></Tracking>
              <Tracking event="complete"><![CDATA[https://ad.widya.id/api/telemetry/complete?id={ad_id}]]></Tracking>
              <Tracking event="skip"><![CDATA[https://ad.widya.id/api/telemetry/skip?id={ad_id}]]></Tracking>
            </TrackingEvents>
            <VideoClicks>
              <ClickThrough id="WIDYA_CLICK_{ad_id}"><![CDATA[{cta_url}]]></ClickThrough>
              <ClickTracking><![CDATA[https://ad.widya.id/api/telemetry/click?id={ad_id}]]></ClickTracking>
            </VideoClicks>
            <MediaFiles>
              <MediaFile id="WIDYA_MEDIA_HD" delivery="progressive" type="video/mp4" width="1920" height="1080" bitrate="2500" scalable="true" maintainAspectRatio="true">
                <![CDATA[{media_url}]]>
              </MediaFile>
            </MediaFiles>
          </Linear>
        </Creative>
      </Creatives>
    </InLine>
  </Ad>
</VAST>"#,
            ad_id = creative.id,
            title = escape_xml(&creative.title),
            desc = escape_xml(&creative.description),
            advertiser = escape_xml(&creative.advertiser_name),
            skip_offset_str = skip_offset_str,
            duration_str = duration_str,
            cta_url = creative.cta_url,
            media_url = creative.media_url
        )
    }

    /// Hasilkan Jadwal VMAP (Video Multiple Ad Playlist) dengan Penanda Waktu Mid-Roll
    pub fn generate_vmap_xml(ads: &[AdCreative]) -> String {
        let mut ad_breaks = String::new();

        for (idx, ad) in ads.iter().enumerate() {
            let (time_offset, break_type) = match &ad.format {
                AdFormat::LinearPreRoll { .. } => ("start".to_string(), "linear"),
                AdFormat::LinearMidRoll { cue_time_sec, .. } => (format!("00:00:{:02}", cue_time_sec), "linear"),
                AdFormat::LinearPostRoll { .. } => ("end".to_string(), "linear"),
                AdFormat::VideoOverlayBanner { .. } => ("00:00:05".to_string(), "nonlinear"),
                _ => continue,
            };

            let vast_xml = Self::generate_vast_xml(ad);

            ad_breaks.push_str(&format!(
r#"  <vmap:AdBreak timeOffset="{time_offset}" breakType="{break_type}" breakId="brk_{idx}">
    <vmap:AdSource id="adsrc_{idx}" allowMultipleAds="false" followRedirects="true">
      <vmap:VASTData>
{vast_xml}
      </vmap:VASTData>
    </vmap:AdSource>
  </vmap:AdBreak>
"#,
                time_offset = time_offset,
                break_type = break_type,
                idx = idx,
                vast_xml = vast_xml.lines().map(|l| format!("        {}", l)).collect::<Vec<_>>().join("\n")
            ));
        }

        format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<vmap:VMAP xmlns:vmap="http://www.iab.net/videosuite/vmap" version="1.0">
{ad_breaks}</vmap:VMAP>"#,
            ad_breaks = ad_breaks
        )
    }
}

fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
     .replace("<", "&lt;")
     .replace(">", "&gt;")
     .replace("\"", "&quot;")
     .replace("'", "&apos;")
}
