//! Unit & Integration Test: WidyaMaterials Pure Crystallography & XRD Engine
//! Memverifikasi 7 sistem kristal Bravais, kalkulasi volume sel satuan,
//! jarak antar-bidang kisi (d_hkl), simulasi pola difraksi sinar-X (XRD) berbasis Hukum Bragg,
//! sintesis difraktogram kontinu 1D, dan konversi ke `CrystalLattice` DFT legacy.

use widya::materials::pure_materials::{
    BasisAtom, CrystalSystem, PureUnitCell, XrdSimulator,
};
use widya::materials::CrystalLattice;

#[test]
fn test_bravais_crystal_systems_volume_and_d_spacing() {
    // 1. Uji Sel Satuan Silikon (Diamond Cubic, a = 5.4309 Å)
    // 8 atom Si dalam sel konvensional FCC berdasar
    let si_atoms = vec![
        BasisAtom::new("Si", 14, 0.0, 0.0, 0.0),
        BasisAtom::new("Si", 14, 0.5, 0.5, 0.0),
        BasisAtom::new("Si", 14, 0.5, 0.0, 0.5),
        BasisAtom::new("Si", 14, 0.0, 0.5, 0.5),
        BasisAtom::new("Si", 14, 0.25, 0.25, 0.25),
        BasisAtom::new("Si", 14, 0.75, 0.75, 0.25),
        BasisAtom::new("Si", 14, 0.75, 0.25, 0.75),
        BasisAtom::new("Si", 14, 0.25, 0.75, 0.75),
    ];
    let silicon = PureUnitCell::new_cubic(5.4309, si_atoms);
    assert_eq!(silicon.crystal_system, CrystalSystem::Cubic);

    // Volume kubik: V = a^3 = 5.4309^3 ≈ 160.18 Å³
    let v_si = silicon.volume();
    assert!((v_si - 160.18).abs() < 0.1);

    // Interplanar spacing:
    // d_111 = a / sqrt(1^2 + 1^2 + 1^2) = 5.4309 / sqrt(3) ≈ 3.1355 Å
    let d_111 = silicon.d_spacing(1, 1, 1).expect("d_111 harus valid");
    assert!((d_111 - 3.1355).abs() < 1e-3);

    // d_220 = a / sqrt(4 + 4 + 0) = 5.4309 / sqrt(8) ≈ 1.9201 Å
    let d_220 = silicon.d_spacing(2, 2, 0).expect("d_220 harus valid");
    assert!((d_220 - 1.9201).abs() < 1e-3);

    // 2. Uji Sel Satuan Tetragonal (Rutil TiO2, a = 4.5937 Å, c = 2.9587 Å)
    let rutile = PureUnitCell::new_tetragonal(4.5937, 2.9587, vec![]);
    assert_eq!(rutile.crystal_system, CrystalSystem::Tetragonal);
    // V = a^2 * c = 4.5937^2 * 2.9587 ≈ 62.43 Å³
    assert!((rutile.volume() - 62.43).abs() < 0.1);
    // d_110 = a / sqrt(2) ≈ 3.248 Å
    let d_110 = rutile.d_spacing(1, 1, 0).unwrap();
    assert!((d_110 - (4.5937 / 2.0f64.sqrt())).abs() < 1e-3);

    // 3. Uji Sel Satuan Heksagonal (Grafit/Grafena, a = 2.464 Å, c = 6.711 Å, gamma = 120°)
    let graphite = PureUnitCell::new_hexagonal(2.464, 6.711, vec![]);
    assert_eq!(graphite.crystal_system, CrystalSystem::Hexagonal);
    // V = (sqrt(3)/2) * a^2 * c ≈ 0.866025 * 6.0713 * 6.711 ≈ 35.29 Å³
    assert!((graphite.volume() - 35.29).abs() < 0.2);
    // d_002 = c / 2 ≈ 3.3555 Å (interlayer graphene spacing)
    let d_002 = graphite.d_spacing(0, 0, 2).unwrap();
    assert!((d_002 - (6.711 / 2.0)).abs() < 1e-3);
}

#[test]
fn test_xrd_bragg_peaks_simulation_and_diffractogram() {
    // Si Diamond Cubic
    let si_atoms = vec![
        BasisAtom::new("Si", 14, 0.0, 0.0, 0.0),
        BasisAtom::new("Si", 14, 0.5, 0.5, 0.0),
        BasisAtom::new("Si", 14, 0.5, 0.0, 0.5),
        BasisAtom::new("Si", 14, 0.0, 0.5, 0.5),
        BasisAtom::new("Si", 14, 0.25, 0.25, 0.25),
        BasisAtom::new("Si", 14, 0.75, 0.75, 0.25),
        BasisAtom::new("Si", 14, 0.75, 0.25, 0.75),
        BasisAtom::new("Si", 14, 0.25, 0.75, 0.75),
    ];
    let silicon = PureUnitCell::new_cubic(5.4309, si_atoms);

    // XRD Cu-Ka1 radiation (lambda = 1.54056 A)
    let sim = XrdSimulator::new(1.54056, 15.0, 80.0);
    let peaks = sim.calculate_peaks(&silicon, 4);

    assert!(!peaks.is_empty(), "Harus ada puncak Bragg dalam rentang 15°-80°");

    // Pada silikon (FCC diamond structure), puncak karakteristik utama adalah:
    // Si (111): 2θ ≈ 28.4° (karena d ≈ 3.135 Å -> sin θ = 1.54056 / (2 * 3.135) = 0.2457 -> θ ≈ 14.22° -> 2θ ≈ 28.44°)
    let peak_111 = peaks.iter().find(|p| p.hkl == (1, 1, 1));
    assert!(peak_111.is_some(), "Puncak Si (111) harus ditemukan");
    let p_111 = peak_111.unwrap();
    assert!((p_111.two_theta_deg - 28.44).abs() < 0.2);

    // Puncak Si (220): 2θ ≈ 47.3°
    let peak_220 = peaks.iter().find(|p| p.hkl == (2, 2, 0));
    assert!(peak_220.is_some(), "Puncak Si (220) harus ditemukan");
    let p_220 = peak_220.unwrap();
    assert!((p_220.two_theta_deg - 47.30).abs() < 0.3);

    // Sintesis spektrum kontinu 1D dengan profil puncak Gaussian
    let (two_theta_axis, intensity_axis) = sim.generate_diffractogram_spectrum(&peaks, 0.2, 0.1);
    assert_eq!(two_theta_axis.len(), intensity_axis.len());
    assert!(two_theta_axis.len() > 100);

    // Verifikasi bahwa intensitas di sekitar 2θ = 28.4° memiliki puncak signifikan
    let idx_near_28_4 = two_theta_axis
        .iter()
        .position(|&x| (x - 28.44).abs() < 0.1)
        .expect("Indeks dekat 28.4° harus ada");
    assert!(intensity_axis[idx_near_28_4] > 10.0);
}

#[test]
fn test_materials_legacy_crystal_lattice_bridge() {
    let atoms = vec![
        BasisAtom::new("Li", 3, 0.0, 0.0, 0.0),
        BasisAtom::new("Fe", 26, 0.5, 0.5, 0.0),
        BasisAtom::new("P", 15, 0.25, 0.25, 0.5),
        BasisAtom::new("O", 8, 0.75, 0.75, 0.5),
    ];
    let cell = PureUnitCell::new_orthorhombic(10.33, 6.01, 4.69, atoms);

    // Konversi ke CrystalLattice DFT legacy
    let legacy_lattice: CrystalLattice = cell.clone().into();
    assert_eq!(legacy_lattice.a_angstrom, 10.33);
    assert_eq!(legacy_lattice.b_angstrom, 6.01);
    assert_eq!(legacy_lattice.c_angstrom, 4.69);
    assert_eq!(legacy_lattice.atom_elements.len(), 4);
    assert_eq!(legacy_lattice.atom_elements[0], "Li");
    assert_eq!(legacy_lattice.atom_elements[1], "Fe");

    // Konversi kembali dari CrystalLattice ke PureUnitCell
    let roundtrip_cell: PureUnitCell = legacy_lattice.into();
    assert_eq!(roundtrip_cell.crystal_system, CrystalSystem::Orthorhombic);
    assert_eq!(roundtrip_cell.atoms.len(), 4);
    assert_eq!(roundtrip_cell.atoms[1].atomic_number, 26); // Fe
}
