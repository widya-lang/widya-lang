//! Unit & Integration Test: WidyaMining Pure SEG-Y & Subsurface Geophysics Engine
//! Memverifikasi konversi format floating point IBM 360/370, transkoding EBCDIC,
//! enkoding/dekoding biner SEG-Y Rev 1/2, perhitungan impedansi akustik dan koefisien refleksi Zoeppritz,
//! serta sintesis jejak seismik dengan wavelet Ricker.

use widya::mining::pure_mining::{
    EbcdicTranscoder, IbmFloat, PureSegyFile, PureTrace, PureTraceHeader, SegyDataFormat,
    SeismicModelingEngine, SubsurfaceLayer,
};
use widya::mining::SegyDataset;

#[test]
fn test_ibm_floating_point_roundtrip() {
    let test_values = [0.0f32, 1.0f32, -1.0f32, 0.5f32, -0.25f32, 1024.0f32, -512.125f32];

    for &val in &test_values {
        let ibm_bytes = IbmFloat::from_f32(val);
        let decoded = IbmFloat::to_f32(ibm_bytes);
        assert!(
            (decoded - val).abs() < 1e-4,
            "IBM float roundtrip gagal untuk {}: hasil={}",
            val,
            decoded
        );
    }
}

#[test]
fn test_ebcdic_ascii_transcoder() {
    let original = "WIDYA-LANG MINING SURVEY 2026";
    let ebcdic = EbcdicTranscoder::encode_ebcdic(original, 3200);
    assert_eq!(ebcdic.len(), 3200);
    // EBCDIC spasi adalah 0x40
    assert_eq!(ebcdic[original.len()], 0x40);

    let decoded = EbcdicTranscoder::decode_ebcdic(&ebcdic[0..original.len()]);
    assert_eq!(decoded, original);
}

#[test]
fn test_segy_binary_roundtrip_ieee_and_ibm() {
    // 1. Buat Berkas SEG-Y dengan format IEEE Float32
    let mut segy_ieee = PureSegyFile::new(2000, 4, SegyDataFormat::IeeeFloat32); // 2000 us = 2 ms, 4 sampel per jejak

    let header1 = PureTraceHeader {
        trace_seq_line: 1,
        trace_seq_file: 1,
        field_record_num: 101,
        trace_num_in_record: 1,
        source_x: 500_000,
        source_y: 9_200_000,
        group_x: 500_050,
        group_y: 9_200_000,
        coord_scalar: 1,
        sample_count: 4,
        sample_interval_us: 2000,
    };
    let trace1 = PureTrace {
        header: header1,
        amplitudes: vec![0.0, 0.75, -0.5, 0.125],
    };
    segy_ieee.traces.push(trace1);

    let bytes_ieee = segy_ieee.encode_bytes();
    // 3200 (text) + 400 (reel) + 240 (tr_hdr) + 4 * 4 (samples) = 3856 bytes
    assert_eq!(bytes_ieee.len(), 3856);

    // Dekode kembali
    let parsed_ieee = PureSegyFile::parse_bytes(&bytes_ieee).expect("Parse SEG-Y IEEE failed");
    assert_eq!(parsed_ieee.sample_interval_us, 2000);
    assert_eq!(parsed_ieee.traces.len(), 1);
    assert_eq!(parsed_ieee.traces[0].header.source_x, 500_000);
    assert_eq!(parsed_ieee.traces[0].header.group_x, 500_050);
    assert_eq!(parsed_ieee.traces[0].amplitudes.len(), 4);
    assert!((parsed_ieee.traces[0].amplitudes[1] - 0.75).abs() < 1e-4);
    assert!((parsed_ieee.traces[0].amplitudes[2] - (-0.5)).abs() < 1e-4);

    // 2. Buat Berkas SEG-Y dengan format warisan IBM Float32
    let mut segy_ibm = PureSegyFile::new(1000, 2, SegyDataFormat::IbmFloat32);
    let trace2 = PureTrace {
        header: PureTraceHeader {
            trace_seq_line: 2,
            trace_seq_file: 2,
            field_record_num: 102,
            trace_num_in_record: 1,
            source_x: 100,
            source_y: 200,
            group_x: 110,
            group_y: 200,
            coord_scalar: 1,
            sample_count: 2,
            sample_interval_us: 1000,
        },
        amplitudes: vec![1.5, -2.25],
    };
    segy_ibm.traces.push(trace2);

    let bytes_ibm = segy_ibm.encode_bytes();
    let parsed_ibm = PureSegyFile::parse_bytes(&bytes_ibm).expect("Parse SEG-Y IBM failed");
    assert_eq!(parsed_ibm.traces.len(), 1);
    assert!((parsed_ibm.traces[0].amplitudes[0] - 1.5).abs() < 1e-3);
    assert!((parsed_ibm.traces[0].amplitudes[1] - (-2.25)).abs() < 1e-3);
}

#[test]
fn test_subsurface_acoustic_impedance_and_seismic_trace_synthesis() {
    // Lapisan 1: Sedimen Kuarter / Shale (Vp = 2000 m/s, rho = 2200 kg/m^3 -> Z1 = 4.4 x 10^6)
    let shale = SubsurfaceLayer {
        thickness_m: 200.0,
        p_wave_velocity_m_s: 2000.0,
        density_kg_m3: 2200.0,
    };
    // Lapisan 2: Batugamping Padat / Limestone (Vp = 4000 m/s, rho = 2600 kg/m^3 -> Z2 = 10.4 x 10^6)
    let limestone = SubsurfaceLayer {
        thickness_m: 300.0,
        p_wave_velocity_m_s: 4000.0,
        density_kg_m3: 2600.0,
    };

    // Koefisien refleksi batas Shale/Limestone:
    // Z1 = 4.4e6, Z2 = 10.4e6
    // R = (10.4 - 4.4) / (10.4 + 4.4) = 6.0 / 14.8 ≈ 0.4054
    let rc = SeismicModelingEngine::reflection_coefficient(&shale, &limestone);
    assert!((rc - 0.4054).abs() < 1e-3);

    // Sintesis jejak seismik 1D (2 ms sampling, 500 ms total, 30 Hz wavelet)
    let layers = vec![shale, limestone];
    let trace = SeismicModelingEngine::synthesize_trace(&layers, 2.0, 500.0, 30.0);
    assert_eq!(trace.len(), 250);

    // Waktu tempuh bolak-balik batas lapisan 1: TWT = 2 * 200m / 2000m/s = 0.2 detik = 200 ms
    // Sampel ke-100 (100 * 2 ms = 200 ms) harus memiliki respon refleksi positif kuat
    let sample_200ms = trace[100];
    assert!(sample_200ms > 0.2, "Puncak refleksi pada 200ms harus positif dan signifikan");
}

#[test]
fn test_mining_legacy_segy_dataset_bridge() {
    let mut segy = PureSegyFile::new(2000, 3, SegyDataFormat::IeeeFloat32);
    segy.traces.push(PureTrace {
        header: PureTraceHeader {
            trace_seq_line: 1,
            trace_seq_file: 1,
            field_record_num: 1,
            trace_num_in_record: 1,
            source_x: 1000,
            source_y: 2000,
            group_x: 1050,
            group_y: 2000,
            coord_scalar: 1,
            sample_count: 3,
            sample_interval_us: 2000,
        },
        amplitudes: vec![0.1, 0.2, 0.3],
    });

    let legacy_ds: SegyDataset = segy.into();
    assert_eq!(legacy_ds.binary_header.sample_interval_us, 2000);
    assert_eq!(legacy_ds.traces.len(), 1);
    assert_eq!(legacy_ds.traces[0].source_x, 1000);
    assert_eq!(legacy_ds.traces[0].samples.len(), 3);
}
