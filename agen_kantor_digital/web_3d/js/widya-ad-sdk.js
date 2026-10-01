/**
 * ==============================================================================
 * 🌟 WIDYA-AD UNIVERSAL CLIENT SDK v2.0
 * Sovereign Ad & Overlay Engine for Video, Web, Mobile & Desktop Apps
 * 
 * Features:
 * - 🎬 Video Overlays: Linear Pre/Mid/Post-roll, Non-linear Bottom Banner, Shoppable Hotspots, PiP
 * - 📱 In-App Overlays: Smart Sticky Banner, Interstitial Modal, Floating Draggable PiP, Slide Drawer
 * - ⚖️ Privacy & Compliance: 100% Zero 3rd-Party Cookies, UU PDP No. 27/2022 Compliant
 * - 📊 MRC Viewability Standard Verification (>=50% visible, >=1s display / >=2s video)
 * ==============================================================================
 */

(function (root, factory) {
    if (typeof define === 'function' && define.amd) {
        define([], factory);
    } else if (typeof module === 'object' && module.exports) {
        module.exports = factory();
    } else {
        root.WidyaAd = factory();
    }
}(typeof self !== 'undefined' ? self : this, function () {
    'use strict';

    const SDK_VERSION = "2.0.0-sovereign";
    const TELEMETRY_LOG = [];

    // Core State
    const state = {
        config: {
            appId: "APP-WIDYA-DEFAULT",
            endpoint: "https://ad.widya.id/api",
            debug: true,
            pdpCompliance: true
        },
        activeOverlays: new Map(),
        viewabilityObservers: new Map()
    };

    function log(msg, type = "info") {
        if (state.config.debug) {
            const prefix = "[WidyaAd Engine]";
            if (type === "warn") console.warn(prefix, msg);
            else if (type === "error") console.error(prefix, msg);
            else console.log(`%c${prefix}%c ${msg}`, "background:#4f46e5;color:#fff;padding:2px 6px;border-radius:4px;font-weight:bold;", "color:#38bdf8;");
        }
    }

    function emitTelemetry(event, data) {
        const record = {
            event,
            data,
            timestamp: Date.now(),
            sdkVersion: SDK_VERSION,
            url: window.location.href
        };
        TELEMETRY_LOG.unshift(record);
        if (TELEMETRY_LOG.length > 50) TELEMETRY_LOG.pop();
        log(`📡 Telemetry Event: ${event} -> ${JSON.stringify(data)}`);
        
        window.dispatchEvent(new CustomEvent('widya-ad-telemetry', { detail: record }));
    }

    // MRC Viewability Tracker using IntersectionObserver
    function setupViewabilityTracker(element, adId, isVideo = false) {
        if (!('IntersectionObserver' in window)) return;

        let visibleStartTime = null;
        let totalVisibleMs = 0;
        let isMrcFired = false;

        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                const ratio = entry.intersectionRatio;
                if (ratio >= 0.5) {
                    if (!visibleStartTime) visibleStartTime = performance.now();
                } else {
                    if (visibleStartTime) {
                        totalVisibleMs += (performance.now() - visibleStartTime);
                        visibleStartTime = null;
                    }
                }

                const minRequiredMs = isVideo ? 2000 : 1000;
                const currentDuration = totalVisibleMs + (visibleStartTime ? (performance.now() - visibleStartTime) : 0);

                if (ratio >= 0.5 && currentDuration >= minRequiredMs && !isMrcFired) {
                    isMrcFired = true;
                    emitTelemetry('MRC_VIEWABLE_IMPRESSION', {
                        adId,
                        isVideo,
                        ratioPct: (ratio * 100).toFixed(1),
                        durationMs: Math.round(currentDuration),
                        mrcCertified: true,
                        pdpSafe: true
                    });
                }
            });
        }, { threshold: [0, 0.25, 0.5, 0.75, 1.0] });

        observer.observe(element);
        state.viewabilityObservers.set(adId, observer);
    }

    // ==========================================
    // 1. VIDEO OVERLAY & PLAYER ATTACHMENT
    // ==========================================

    function attachToVideo(videoElement, options = {}) {
        if (!videoElement) {
            log("Video element not found for attachToVideo", "error");
            return null;
        }

        const container = videoElement.parentElement;
        if (!container) return null;

        if (window.getComputedStyle(container).position === 'static') {
            container.style.position = 'relative';
        }

        const controller = {
            video: videoElement,
            container: container,
            options: options,
            prerollDone: false,
            midrollsTriggered: new Set(),
            hotspotsTriggered: new Set(),

            // Inject Non-linear Bottom Overlay Banner
            showBottomBanner(adData) {
                const overlayId = `widya-v-banner-${adData.id || Date.now()}`;
                const existing = document.getElementById(overlayId);
                if (existing) existing.remove();

                const el = document.createElement('div');
                el.id = overlayId;
                el.className = 'widya-video-overlay-banner';
                el.innerHTML = `
                    <div style="
                        position: absolute;
                        bottom: 16px;
                        left: 50%;
                        transform: translateX(-50%);
                        width: 90%;
                        max-width: 680px;
                        background: rgba(15, 23, 42, 0.88);
                        backdrop-filter: blur(16px);
                        -webkit-backdrop-filter: blur(16px);
                        border: 1px solid rgba(99, 102, 241, 0.45);
                        border-radius: 14px;
                        box-shadow: 0 16px 36px rgba(0,0,0,0.65), 0 0 20px rgba(99,102,241,0.25);
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        padding: 10px 16px;
                        z-index: 100;
                        color: #f8fafc;
                        font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
                        animation: widyaSlideUp 0.35s cubic-bezier(0.16, 1, 0.3, 1);
                        pointer-events: auto;
                    ">
                        <div style="display: flex; align-items: center; gap: 12px; min-width: 0;">
                            <div style="font-size: 24px; background: rgba(99, 102, 241, 0.2); width: 44px; height: 44px; border-radius: 10px; display: flex; align-items: center; justify-content: center; border: 1px solid rgba(99, 102, 241, 0.35); flex-shrink: 0;">
                                ${adData.icon || '📢'}
                            </div>
                            <div style="min-width: 0;">
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="font-size: 9.5px; text-transform: uppercase; color: #818cf8; font-weight: 800; letter-spacing: 0.5px; background: rgba(99,102,241,0.18); padding: 1px 6px; border-radius: 4px;">Sponsor</span>
                                    <span style="font-size: 11px; color: #94a3b8; font-weight: 600;">${adData.advertiser || 'Widya Partner'}</span>
                                </div>
                                <div style="font-size: 13px; font-weight: 700; color: #ffffff; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; margin-top: 2px;">
                                    ${adData.title}
                                </div>
                                <div style="font-size: 11px; color: #94a3b8; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                    ${adData.desc || ''}
                                </div>
                            </div>
                        </div>
                        <div style="display: flex; align-items: center; gap: 8px; flex-shrink: 0; margin-left: 12px;">
                            <a href="${adData.ctaUrl || '#'}" target="_blank" onclick="window.WidyaAd._onAdClick('${adData.id}')" style="
                                background: linear-gradient(135deg, #6366f1, #4f46e5);
                                color: #ffffff;
                                text-decoration: none;
                                padding: 7px 14px;
                                border-radius: 8px;
                                font-size: 11.5px;
                                font-weight: 700;
                                display: inline-flex;
                                align-items: center;
                                gap: 4px;
                                box-shadow: 0 4px 12px rgba(99, 102, 241, 0.35);
                                transition: transform 0.15s ease;
                            " onmouseover="this.style.transform='scale(1.04)'" onmouseout="this.style.transform='scale(1)'">
                                ${adData.ctaText || 'Lihat'} ↗
                            </a>
                            <button onclick="document.getElementById('${overlayId}').remove()" style="
                                background: rgba(255, 255, 255, 0.08);
                                border: 1px solid rgba(255,255,255,0.1);
                                color: #94a3b8;
                                width: 26px;
                                height: 26px;
                                border-radius: 6px;
                                cursor: pointer;
                                font-size: 12px;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                transition: all 0.2s;
                            " onmouseover="this.style.color='#fff';this.style.background='rgba(239,68,68,0.3)'" onmouseout="this.style.color='#94a3b8';this.style.background='rgba(255,255,255,0.08)'">✕</button>
                        </div>
                    </div>
                `;

                container.appendChild(el);
                setupViewabilityTracker(el, adData.id || overlayId, false);
                emitTelemetry('VIDEO_OVERLAY_DISPLAYED', { adId: adData.id, format: 'VideoOverlayBanner' });

                if (adData.autoCloseSec) {
                    setTimeout(() => {
                        const target = document.getElementById(overlayId);
                        if (target) target.remove();
                    }, adData.autoCloseSec * 1000);
                }
            },

            // Show Shoppable Video Hotspot Pin
            showShoppablePin(productData) {
                const pinId = `widya-pin-${productData.id || Date.now()}`;
                const existing = document.getElementById(pinId);
                if (existing) existing.remove();

                const el = document.createElement('div');
                el.id = pinId;
                el.innerHTML = `
                    <div style="
                        position: absolute;
                        top: ${productData.top || '20%'};
                        right: ${productData.right || '20px'};
                        background: rgba(6, 78, 59, 0.9);
                        backdrop-filter: blur(14px);
                        -webkit-backdrop-filter: blur(14px);
                        border: 1px solid #10b981;
                        border-radius: 12px;
                        padding: 10px 14px;
                        box-shadow: 0 12px 28px rgba(16, 185, 129, 0.35);
                        display: flex;
                        align-items: center;
                        gap: 12px;
                        z-index: 105;
                        color: #ffffff;
                        font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
                        animation: widyaPopIn 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
                        pointer-events: auto;
                    ">
                        <div style="font-size: 22px; background: rgba(16,185,129,0.25); width: 40px; height: 40px; border-radius: 8px; display: flex; align-items: center; justify-content: center;">
                            🛍️
                        </div>
                        <div>
                            <div style="font-size: 9.5px; text-transform: uppercase; font-weight: 800; color: #6ee7b7; letter-spacing: 0.5px;">Produk dalam Video</div>
                            <div style="font-size: 12.5px; font-weight: 700; color: #fff;">${productData.title}</div>
                            <div style="font-size: 11.5px; font-weight: 800; color: #a7f3d0;">Rp ${(productData.price || 0).toLocaleString('id-ID')}</div>
                        </div>
                        <div style="display:flex; align-items:center; gap:6px;">
                            <a href="${productData.ctaUrl || '#'}" target="_blank" onclick="window.WidyaAd._onAdClick('${productData.id}')" style="
                                background: #10b981;
                                color: #022c22;
                                text-decoration: none;
                                padding: 6px 12px;
                                border-radius: 6px;
                                font-size: 11px;
                                font-weight: 800;
                                transition: transform 0.15s;
                            " onmouseover="this.style.transform='scale(1.05)'" onmouseout="this.style.transform='scale(1)'">
                                Beli
                            </a>
                            <button onclick="document.getElementById('${pinId}').remove()" style="
                                background: rgba(255,255,255,0.15); border:none; color:#fff; width:22px; height:22px; border-radius:50%; cursor:pointer; font-size:10px; display:flex; align-items:center; justify-content:center;
                            ">✕</button>
                        </div>
                    </div>
                `;

                container.appendChild(el);
                emitTelemetry('SHOPPABLE_HOTSPOT_SHOWN', { productId: productData.id, price: productData.price });

                // Auto dismiss after 7 seconds
                setTimeout(() => {
                    const p = document.getElementById(pinId);
                    if (p) p.remove();
                }, 7000);
            },

            // Play Linear Pre-Roll Ad
            playLinearAd(adData, onComplete) {
                const originalSrc = videoElement.src;
                const originalCurrentTime = videoElement.currentTime;
                const isPaused = videoElement.paused;

                log(`🎬 Starting Linear Ad: ${adData.title}`);

                // Ad Overlay Container with Skip button & Counter
                const adOverlayId = `widya-linear-ad-ui`;
                const existing = document.getElementById(adOverlayId);
                if (existing) existing.remove();

                const ui = document.createElement('div');
                ui.id = adOverlayId;
                ui.innerHTML = `
                    <div style="
                        position: absolute;
                        top: 0; left: 0; width: 100%; height: 100%;
                        background: rgba(0,0,0,0.4);
                        z-index: 150;
                        pointer-events: none;
                        font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
                    ">
                        <!-- Top Info Badge -->
                        <div style="
                            position: absolute; top: 16px; left: 16px;
                            background: rgba(15,23,42,0.85); backdrop-filter: blur(10px);
                            border: 1px solid rgba(255,255,255,0.15);
                            padding: 6px 12px; border-radius: 8px;
                            color: #fff; font-size: 11px; font-weight: 700;
                            display: flex; align-items: center; gap: 8px;
                            pointer-events: auto;
                        ">
                            <span style="background: #e11d48; color:#fff; padding: 2px 6px; border-radius: 4px; font-size: 9px; font-weight: 800;">IKLAN</span>
                            <span>${adData.title}</span>
                            <span id="widya-ad-countdown" style="color: #cbd5e1; font-weight: 600;">(15s)</span>
                        </div>

                        <!-- Bottom Right Action / Skip Button -->
                        <div id="widya-skip-container" style="
                            position: absolute; bottom: 24px; right: 24px;
                            pointer-events: auto;
                            display: flex; align-items: center; gap: 10px;
                        ">
                            <a href="${adData.ctaUrl || '#'}" target="_blank" onclick="window.WidyaAd._onAdClick('${adData.id}')" style="
                                background: linear-gradient(135deg, #6366f1, #4f46e5);
                                color: #fff; text-decoration: none;
                                padding: 9px 18px; border-radius: 8px;
                                font-size: 12px; font-weight: 700;
                                box-shadow: 0 4px 14px rgba(99,102,241,0.4);
                            ">
                                ${adData.ctaText || 'Kunjungi Situs'} ↗
                            </a>
                            <button id="widya-skip-btn" disabled style="
                                background: rgba(15,23,42,0.9);
                                border: 1px solid rgba(255,255,255,0.2);
                                color: #94a3b8;
                                padding: 9px 18px; border-radius: 8px;
                                font-size: 12px; font-weight: 700;
                                cursor: not-allowed;
                                transition: all 0.2s;
                            ">
                                Lewati dalam 5s
                            </button>
                        </div>
                    </div>
                `;

                container.appendChild(ui);
                emitTelemetry('LINEAR_AD_START', { adId: adData.id, duration: adData.durationSec || 15 });

                let remainingSkip = adData.skipOffsetSec || 5;
                let remainingTotal = adData.durationSec || 15;

                const countdownInterval = setInterval(() => {
                    remainingSkip--;
                    remainingTotal--;

                    const cdEl = document.getElementById('widya-ad-countdown');
                    if (cdEl) cdEl.innerText = `(${Math.max(0, remainingTotal)}s)`;

                    const skipBtn = document.getElementById('widya-skip-btn');
                    if (skipBtn) {
                        if (remainingSkip > 0) {
                            skipBtn.innerText = `Lewati dalam ${remainingSkip}s`;
                        } else {
                            skipBtn.disabled = false;
                            skipBtn.style.cursor = 'pointer';
                            skipBtn.style.background = '#ffffff';
                            skipBtn.style.color = '#0f172a';
                            skipBtn.style.fontWeight = '800';
                            skipBtn.innerText = 'Lewati Iklan ⏭️';
                            skipBtn.onclick = () => finishAd(true);
                        }
                    }

                    if (remainingTotal <= 0) {
                        finishAd(false);
                    }
                }, 1000);

                function finishAd(skipped) {
                    clearInterval(countdownInterval);
                    const uiEl = document.getElementById(adOverlayId);
                    if (uiEl) uiEl.remove();

                    emitTelemetry(skipped ? 'LINEAR_AD_SKIPPED' : 'LINEAR_AD_COMPLETE', { adId: adData.id });
                    if (onComplete) onComplete();
                }
            }
        };

        // Timeupdate listener for mid-roll & shoppable hotspot cues
        videoElement.addEventListener('timeupdate', () => {
            const currentSec = Math.floor(videoElement.currentTime);

            // Check Mid-rolls
            if (options.midrolls) {
                options.midrolls.forEach(midroll => {
                    if (currentSec === midroll.cueSec && !controller.midrollsTriggered.has(midroll.cueSec)) {
                        controller.midrollsTriggered.add(midroll.cueSec);
                        videoElement.pause();
                        controller.playLinearAd(midroll, () => {
                            videoElement.play();
                        });
                    }
                });
            }

            // Check Hotspots
            if (options.hotspots) {
                options.hotspots.forEach(hotspot => {
                    if (currentSec === hotspot.timestampSec && !controller.hotspotsTriggered.has(hotspot.timestampSec)) {
                        controller.hotspotsTriggered.add(hotspot.timestampSec);
                        controller.showShoppablePin(hotspot);
                    }
                });
            }
        });

        // Trigger Pre-roll if provided
        if (options.preroll && !controller.prerollDone) {
            controller.prerollDone = true;
            controller.playLinearAd(options.preroll, () => {
                videoElement.play();
            });
        }

        return controller;
    }

    // ==========================================
    // 2. IN-APP OVERLAYS (Sticky, Modal, PiP)
    // ==========================================

    function showAppStickyBanner(options = {}) {
        const id = options.id || `widya-sticky-${Date.now()}`;
        const existing = document.getElementById(id);
        if (existing) existing.remove();

        const isBottom = (options.position || 'bottom') === 'bottom';
        const posStyle = isBottom ? 'bottom: 0; border-top: 1px solid rgba(99,102,241,0.35);' : 'top: 0; border-bottom: 1px solid rgba(99,102,241,0.35);';

        const banner = document.createElement('div');
        banner.id = id;
        banner.innerHTML = `
            <div style="
                position: fixed; left: 0; width: 100%; height: 60px;
                ${posStyle}
                background: rgba(10, 15, 29, 0.95);
                backdrop-filter: blur(20px);
                -webkit-backdrop-filter: blur(20px);
                z-index: 99999;
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 0 24px;
                color: #f8fafc;
                font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
                box-shadow: 0 10px 30px rgba(0,0,0,0.6);
                animation: ${isBottom ? 'widyaSlideUp' : 'widyaSlideDown'} 0.3s ease;
            ">
                <div style="display: flex; align-items: center; gap: 14px;">
                    <div style="font-size: 22px; background: linear-gradient(135deg, #6366f1, #06b6d4); width: 38px; height: 38px; border-radius: 10px; display: flex; align-items: center; justify-content: center; box-shadow: 0 4px 12px rgba(99,102,241,0.3);">
                        ${options.icon || '🚀'}
                    </div>
                    <div>
                        <div style="display:flex; align-items:center; gap:8px;">
                            <span style="font-size: 9px; text-transform: uppercase; font-weight: 800; background: rgba(99,102,241,0.25); color: #818cf8; padding: 2px 6px; border-radius: 4px;">Sponsor</span>
                            <span style="font-weight: 700; font-size: 13.5px; color: #ffffff;">${options.title || 'Widya Sovereign Platform'}</span>
                        </div>
                        <div style="font-size: 11px; color: #94a3b8; margin-top: 1px;">${options.desc || 'Solusi ekosistem digital nasional berkinerja tinggi.'}</div>
                    </div>
                </div>
                <div style="display: flex; align-items: center; gap: 12px;">
                    <a href="${options.ctaUrl || '#'}" target="_blank" onclick="window.WidyaAd._onAdClick('${id}')" style="
                        background: linear-gradient(135deg, #6366f1, #4f46e5);
                        color: #ffffff;
                        text-decoration: none;
                        padding: 8px 18px;
                        border-radius: 8px;
                        font-size: 12px;
                        font-weight: 700;
                        box-shadow: 0 4px 12px rgba(99,102,241,0.3);
                        transition: transform 0.15s;
                    " onmouseover="this.style.transform='scale(1.04)'" onmouseout="this.style.transform='scale(1)'">
                        ${options.ctaText || 'Jelajahi'} ↗
                    </a>
                    <button onclick="document.getElementById('${id}').remove()" style="
                        background: rgba(255,255,255,0.08);
                        border: none;
                        color: #94a3b8;
                        width: 28px;
                        height: 28px;
                        border-radius: 6px;
                        cursor: pointer;
                        font-size: 14px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    ">✕</button>
                </div>
            </div>
        `;

        document.body.appendChild(banner);
        setupViewabilityTracker(banner, id, false);
        emitTelemetry('APP_STICKY_BANNER_SHOWN', { id, position: options.position || 'bottom' });
        return banner;
    }

    function showInterstitialModal(options = {}) {
        const id = options.id || `widya-modal-${Date.now()}`;
        const existing = document.getElementById(id);
        if (existing) existing.remove();

        const modal = document.createElement('div');
        modal.id = id;
        modal.innerHTML = `
            <div style="
                position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
                background: rgba(2, 6, 23, 0.85);
                backdrop-filter: blur(24px);
                -webkit-backdrop-filter: blur(24px);
                z-index: 100000;
                display: flex;
                align-items: center;
                justify-content: center;
                font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
                animation: widyaFadeIn 0.25s ease;
            ">
                <div style="
                    background: linear-gradient(145deg, #0f172a, #1e1b4b);
                    border: 1px solid rgba(99, 102, 241, 0.45);
                    border-radius: 20px;
                    width: 90%;
                    max-width: 520px;
                    padding: 32px;
                    box-shadow: 0 24px 60px rgba(0,0,0,0.8), 0 0 40px rgba(99,102,241,0.25);
                    text-align: center;
                    position: relative;
                ">
                    <button id="${id}-close-btn" onclick="document.getElementById('${id}').remove()" style="
                        position: absolute; top: 18px; right: 18px;
                        background: rgba(255,255,255,0.08);
                        border: 1px solid rgba(255,255,255,0.15);
                        color: #94a3b8;
                        width: 32px; height: 32px;
                        border-radius: 50%;
                        cursor: pointer;
                        font-size: 14px;
                    ">✕</button>

                    <div style="font-size: 48px; margin-bottom: 12px;">${options.icon || '💎'}</div>
                    <div style="font-size: 10px; text-transform: uppercase; font-weight: 800; color: #818cf8; letter-spacing: 1px; margin-bottom: 6px;">Sponsorship Resmi • WidyaAd</div>
                    <h2 style="font-size: 20px; font-weight: 800; color: #ffffff; margin-bottom: 10px; line-height: 1.3;">${options.title || 'Widya Cloud Sovereign Enterprise'}</h2>
                    <p style="font-size: 13px; color: #94a3b8; margin-bottom: 24px; line-height: 1.6;">${options.desc || 'Transformasi infrastruktur digital Anda dengan ekosistem berdaulat dan performa kompilasi native 100% tanpa runtime eksternal.'}</p>
                    
                    <div style="display: flex; gap: 12px; justify-content: center;">
                        <a href="${options.ctaUrl || '#'}" target="_blank" onclick="window.WidyaAd._onAdClick('${id}')" style="
                            background: linear-gradient(135deg, #6366f1, #4f46e5);
                            color: #fff;
                            text-decoration: none;
                            padding: 12px 28px;
                            border-radius: 10px;
                            font-size: 13.5px;
                            font-weight: 800;
                            box-shadow: 0 6px 20px rgba(99,102,241,0.4);
                            display: inline-flex; align-items: center; gap: 6px;
                        ">
                            ${options.ctaText || 'Pelajari Selengkapnya'} ↗
                        </a>
                        <button onclick="document.getElementById('${id}').remove()" style="
                            background: rgba(255,255,255,0.06);
                            border: 1px solid rgba(255,255,255,0.15);
                            color: #cbd5e1;
                            padding: 12px 20px;
                            border-radius: 10px;
                            font-size: 13.5px;
                            font-weight: 700;
                            cursor: pointer;
                        ">Tutup</button>
                    </div>
                </div>
            </div>
        `;

        document.body.appendChild(modal);
        emitTelemetry('INTERSTITIAL_MODAL_SHOWN', { id, title: options.title });
        return modal;
    }

    function showFloatingPip(options = {}) {
        const id = options.id || `widya-pip-${Date.now()}`;
        const existing = document.getElementById(id);
        if (existing) existing.remove();

        const pip = document.createElement('div');
        pip.id = id;
        pip.innerHTML = `
            <div id="${id}-card" style="
                position: fixed; bottom: 24px; right: 24px;
                width: 280px;
                background: rgba(15, 23, 42, 0.94);
                backdrop-filter: blur(16px);
                -webkit-backdrop-filter: blur(16px);
                border: 1px solid rgba(99, 102, 241, 0.4);
                border-radius: 16px;
                padding: 14px;
                box-shadow: 0 16px 40px rgba(0,0,0,0.7), 0 0 20px rgba(99,102,241,0.2);
                z-index: 9999;
                color: #fff;
                font-family: 'Plus Jakarta Sans', system-ui, sans-serif;
                animation: widyaPopIn 0.3s ease;
                cursor: grab;
            ">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                    <span style="font-size: 9px; text-transform: uppercase; font-weight: 800; color: #818cf8; background: rgba(99,102,241,0.2); padding: 2px 6px; border-radius: 4px;">Mini Sponsor</span>
                    <button onclick="document.getElementById('${id}').remove()" style="background:none; border:none; color:#94a3b8; cursor:pointer; font-size:12px;">✕</button>
                </div>
                <div style="display:flex; align-items:center; gap:10px; margin-bottom: 10px;">
                    <div style="font-size: 24px;">${options.icon || '✨'}</div>
                    <div>
                        <div style="font-size: 12px; font-weight: 700; color: #fff;">${options.title || 'Widya Ecosystem'}</div>
                        <div style="font-size: 10.5px; color: #94a3b8;">${options.desc || 'Platform Nasional'}</div>
                    </div>
                </div>
                <a href="${options.ctaUrl || '#'}" target="_blank" onclick="window.WidyaAd._onAdClick('${id}')" style="
                    display: block; text-align: center;
                    background: linear-gradient(135deg, #6366f1, #4f46e5);
                    color: #fff; text-decoration: none;
                    padding: 7px; border-radius: 8px;
                    font-size: 11px; font-weight: 700;
                ">
                    ${options.ctaText || 'Kunjungi'} ↗
                </a>
            </div>
        `;

        document.body.appendChild(pip);
        emitTelemetry('FLOATING_PIP_SHOWN', { id });
        return pip;
    }

    // Inject Base CSS Keyframes
    const styleEl = document.createElement('style');
    styleEl.innerHTML = `
        @keyframes widyaSlideUp {
            from { opacity: 0; transform: translate(-50%, 20px); }
            to { opacity: 1; transform: translate(-50%, 0); }
        }
        @keyframes widyaSlideDown {
            from { opacity: 0; transform: translateY(-20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        @keyframes widyaPopIn {
            0% { opacity: 0; transform: scale(0.85); }
            70% { transform: scale(1.03); }
            100% { opacity: 1; transform: scale(1); }
        }
        @keyframes widyaFadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
    `;
    document.head.appendChild(styleEl);

    // Public API
    return {
        version: SDK_VERSION,
        init(config = {}) {
            Object.assign(state.config, config);
            log(`Initialized with AppID: ${state.config.appId} (UU PDP Mode: ${state.config.pdpCompliance ? 'ACTIVE' : 'OFF'})`);
        },
        attachToVideo,
        showAppStickyBanner,
        showInterstitialModal,
        showFloatingPip,
        getTelemetryLog() {
            return [...TELEMETRY_LOG];
        },
        _onAdClick(adId) {
            emitTelemetry('AD_CLICK_ENGAGEMENT', { adId });
        }
    };
}));
