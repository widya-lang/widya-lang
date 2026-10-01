use widya::agri::{
    AnimalIdTag, EstrusHealthStatus, LivestockSpecies, PrecisionLivestockManager,
};
use widya::fintech::{
    BlackScholesGreeks, FixMessage, FixOrderType, FixSide, ForexMatrixEngine, LimitOrder,
    LimitOrderBook, OrderSide, QuantitativeEngine,
};

#[test]
fn test_smart_livestock_farming() {
    let mut manager = PrecisionLivestockManager::new();

    // Register Sapi Limousin ISO 11784
    let tag = manager.register_animal(
        360123456789,
        LivestockSpecies::BovineCattle,
        "Sapi Limousin Super",
        "2024-03-15",
    );
    assert_eq!(tag.national_id, 360123456789);
    assert!(tag.halal_certified);

    // THI Comfort Test
    let (thi_comfort, cat_comfort) = PrecisionLivestockManager::calculate_thi(24.0, 60.0);
    assert!(thi_comfort < 72.0);
    assert_eq!(cat_comfort, "Comfort / No Stress");

    // Telemetry Day 1: Normal
    let st1 = manager.ingest_telemetry(
        360123456789,
        38.6,
        480,
        1800,
        450.0,
        Some(22.5),
        25.0,
        65.0,
    ).expect("Ingest failed");
    assert_eq!(st1, EstrusHealthStatus::Normal);

    // Telemetry Day 2: Deteksi Birahi (Estrus Spike: Step count > 4000)
    let st2 = manager.ingest_telemetry(
        360123456789,
        38.9,
        420,
        4600,
        451.8,
        Some(21.0),
        26.0,
        65.0,
    ).expect("Ingest failed");
    assert_eq!(st2, EstrusHealthStatus::SuspectedEstrus);

    // ADG Growth Rate Calculation
    let adg = manager.calculate_adg(360123456789).expect("ADG failed");
    assert!((adg - 1.8).abs() < 1e-5); // 1.8 kg/day
}

#[test]
fn test_hft_order_book_matching() {
    let mut book = LimitOrderBook::new("BBCA.JK", 2);

    // Place Sell orders (Asks)
    let sell1 = LimitOrder {
        order_id: 101,
        client_id: "CLIENT_A".to_string(),
        symbol: "BBCA.JK".to_string(),
        side: OrderSide::Ask,
        price: 10000.0,
        quantity: 50.0,
        timestamp_ns: 1000,
    };
    let trades1 = book.place_order(sell1);
    assert!(trades1.is_empty()); // No match yet, added to book

    let sell2 = LimitOrder {
        order_id: 102,
        client_id: "CLIENT_B".to_string(),
        symbol: "BBCA.JK".to_string(),
        side: OrderSide::Ask,
        price: 10025.0,
        quantity: 100.0,
        timestamp_ns: 1001,
    };
    book.place_order(sell2);

    // Check BBO before buy
    let bbo = book.get_bbo();
    assert_eq!(bbo.best_ask_price, Some(10000.0));
    assert_eq!(bbo.best_ask_qty, 50.0);

    // Place aggressive Buy order (Bid) that sweeps top ask and partially fills next
    let buy1 = LimitOrder {
        order_id: 201,
        client_id: "CLIENT_C".to_string(),
        symbol: "BBCA.JK".to_string(),
        side: OrderSide::Bid,
        price: 10025.0,
        quantity: 70.0, // Should fill 50 @ 10000 and 20 @ 10025
        timestamp_ns: 1002,
    };
    let fills = book.place_order(buy1);
    assert_eq!(fills.len(), 2);
    assert_eq!(fills[0].price, 10000.0);
    assert_eq!(fills[0].quantity, 50.0);
    assert_eq!(fills[1].price, 10025.0);
    assert_eq!(fills[1].quantity, 20.0);

    // Check remaining BBO
    let bbo_after = book.get_bbo();
    assert_eq!(bbo_after.best_ask_price, Some(10025.0));
    assert_eq!(bbo_after.best_ask_qty, 80.0); // 100 - 20 = 80
}

#[test]
fn test_fix_protocol_engine() {
    let mut new_order = FixMessage::new("D", "BROKER_CLIENT", "BURSA_EFEK", 101);
    new_order.set_tag(11, "CLORDID_99881"); // ClOrdID
    new_order.set_tag(55, "BBRI.JK");        // Symbol
    new_order.set_tag(54, "1");              // Side: 1 = Buy
    new_order.set_tag(38, "1000");           // OrderQty
    new_order.set_tag(40, "2");              // OrdType: 2 = Limit
    new_order.set_tag(44, "4850.0");         // Price

    let fix_str = new_order.serialize_to_fix_str();
    assert!(fix_str.contains("35=D"));
    assert!(fix_str.contains("55=BBRI.JK"));
    assert!(fix_str.contains("10=")); // Checksum present

    let parsed = FixMessage::parse_fix_str(&fix_str).expect("Parse FIX failed");
    assert_eq!(parsed.msg_type, "D");
    assert_eq!(parsed.get_tag(55).unwrap(), "BBRI.JK");
    assert_eq!(parsed.get_tag(44).unwrap(), "4850.0");
}

#[test]
fn test_forex_matrix_and_carry_trade() {
    let mut fx = ForexMatrixEngine::new();
    fx.update_quote("EUR", "USD", 1.0800, 1.0802, 0.0001);
    fx.update_quote("USD", "IDR", 15800.0, 15810.0, 1.0);

    // Spread in Pips
    let spread = fx.calculate_spread_pips("EUR", "USD").expect("Spread failed");
    assert!((spread - 2.0).abs() < 1e-4); // 2 pips

    // Synthetic Cross Rate EUR/IDR
    let (eur_idr_bid, eur_idr_ask) = fx.calculate_triangular_cross("EUR", "IDR").expect("Cross failed");
    assert!((eur_idr_bid - (1.0800 * 15800.0)).abs() < 1e-2);
    assert!((eur_idr_ask - (1.0802 * 15810.0)).abs() < 1e-2);

    // Carry Trade: IDR/USD (BI 6.00% vs Fed 4.50% -> +1.5% Positive Carry)
    let carry = fx.analyze_carry_trade("IDR", "USD").expect("Carry failed");
    assert_eq!(carry.net_carry_yield_pct, 1.50);
}

#[test]
fn test_quant_vwap_and_black_scholes() {
    // VWAP
    let trades = vec![(100.0, 10.0), (105.0, 20.0), (110.0, 10.0)];
    let vwap = QuantitativeEngine::calculate_vwap(&trades);
    // (1000 + 2100 + 1100) / 40 = 4200 / 40 = 105.0
    assert_eq!(vwap, 105.0);

    // Black-Scholes Call & Put Greeks
    // S=100, K=100, T=1.0 year, r=5% (0.05), sigma=20% (0.20)
    let greeks = QuantitativeEngine::black_scholes_greeks(100.0, 100.0, 1.0, 0.05, 0.20);
    assert!(greeks.call_price > 10.0 && greeks.call_price < 11.0); // ~10.45
    assert!(greeks.delta_call > 0.60 && greeks.delta_call < 0.68); // ~0.637
    assert!(greeks.gamma > 0.0);
    assert!(greeks.vega > 0.0);

    // Parametric VaR (100 Million IDR, 2% daily vol, 99% confidence, 1 day)
    let var = QuantitativeEngine::calculate_parametric_var(100_000_000.0, 2.0, 99.0, 1);
    assert!(var.parametric_var_amount > 4_000_000.0);
}
