// ==============================================================================
// WidyaWeb Sovereign Broadcast Studio - PWA Service Worker (v2.1)
// ==============================================================================
// Menyediakan kapabilitas offline-first, background telemetry sync,
// dan push notifications untuk kru penyiaran OB Van di lapangan.
// ==============================================================================

const CACHE_NAME = 'widya-mcr-studio-v2.1';
const PRECACHE_ASSETS = [
    '/',
    '/index.html',
    '/app.js',
    '/style.css',
    '/kamera',
    '/kamera_hp.html',
    '/var',
    '/var_dewan_hakim.html',
    '/manifest.json'
];

// 1. Install Event: Pra-pengunduhan aset inti ke Cache Storage
self.addEventListener('install', (event) => {
    event.waitUntil(
        caches.open(CACHE_NAME).then((cache) => {
            console.log('[Widya PWA SW] Menyimpan aset inti MCR Studio ke dalam cache...');
            return cache.addAll(PRECACHE_ASSETS).catch((err) => {
                console.warn('[Widya PWA SW] Sebagian aset gagal diprecache (mungkin koneksi dev):', err);
            });
        }).then(() => self.skipWaiting())
    );
});

// 2. Activate Event: Membersihkan cache versi lama
self.addEventListener('activate', (event) => {
    event.waitUntil(
        caches.keys().then((cacheNames) => {
            return Promise.all(
                cacheNames.map((cache) => {
                    if (cache !== CACHE_NAME) {
                        console.log('[Widya PWA SW] Menghapus cache lawas:', cache);
                        return caches.delete(cache);
                    }
                })
            );
        }).then(() => self.clients.claim())
    );
});

// 3. Fetch Event: Strategi hibrida Caching
// - WebSocket & API (/api/*, /ws, /broadcast/*): Langsung ke jaringan (network-only)
// - Aset statis & UI: Stale-While-Revalidate dengan Fallback Offline
self.addEventListener('fetch', (event) => {
    const url = new URL(event.request.url);

    // Jangan cache websocket dan REST API dinamis
    if (url.pathname.startsWith('/ws') || url.pathname.startsWith('/api/') || url.pathname.startsWith('/broadcast/')) {
        return;
    }

    event.respondWith(
        caches.match(event.request).then((cachedResponse) => {
            if (cachedResponse) {
                // Background revalidate untuk pembaruan instan
                fetch(event.request).then((networkResponse) => {
                    if (networkResponse && networkResponse.status === 200) {
                        caches.open(CACHE_NAME).then((cache) => {
                            cache.put(event.request, networkResponse.clone());
                        });
                    }
                }).catch(() => {
                    // Jaringan offline, tetap pakai cachedResponse
                });
                return cachedResponse;
            }

            // Jika belum ada di cache, unduh dari jaringan
            return fetch(event.request).then((networkResponse) => {
                if (!networkResponse || networkResponse.status !== 200 || networkResponse.type !== 'basic') {
                    return networkResponse;
                }
                const responseToCache = networkResponse.clone();
                caches.open(CACHE_NAME).then((cache) => {
                    cache.put(event.request, responseToCache);
                });
                return networkResponse;
            }).catch(() => {
                // Fallback jika offline total dan aset belum ada di cache
                if (event.request.destination === 'document') {
                    return caches.match('/');
                }
            });
        })
    );
});

// 4. Push Notification Event: Notifikasi darurat dari Sutradara MCR ke Operator
self.addEventListener('push', (event) => {
    let payload = {
        title: '📡 Widya MCR Broadcast Alert',
        body: 'Instruksi baru dari sutradara siaran!',
        tag: 'mcr-alert',
        icon: '/manifest.json'
    };

    if (event.data) {
        try {
            payload = event.data.json();
        } catch (e) {
            payload.body = event.data.text();
        }
    }

    const options = {
        body: payload.body,
        icon: payload.icon || 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><circle cx="50" cy="50" r="45" fill="%23e11d48"/><polygon points="40,30 75,50 40,70" fill="white"/></svg>',
        badge: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><circle cx="50" cy="50" r="45" fill="%2322c55e"/></svg>',
        vibrate: [200, 100, 200, 100, 200],
        data: payload.data || { url: '/' },
        actions: [
            { action: 'open_app', title: 'Buka Antarmuka' },
            { action: 'dismiss', title: 'Abaikan' }
        ]
    };

    event.waitUntil(
        self.registration.showNotification(payload.title, options)
    );
});

// 5. Notification Click Event: Fokus atau buka window MCR
self.addEventListener('notificationclick', (event) => {
    event.notification.close();
    if (event.action === 'dismiss') return;

    const targetUrl = event.notification.data && event.notification.data.url ? event.notification.data.url : '/';
    event.waitUntil(
        clients.matchAll({ type: 'window', includeUncontrolled: true }).then((windowClients) => {
            for (let client of windowClients) {
                if (client.url.includes(targetUrl) && 'focus' in client) {
                    return client.focus();
                }
            }
            if (clients.openWindow) {
                return clients.openWindow(targetUrl);
            }
        })
    );
});

// 6. Resilient Background Sync: Mengirim telemetri tertunda saat jaringan pulih
self.addEventListener('sync', (event) => {
    if (event.tag === 'sync-offline-telemetry') {
        console.log('[Widya PWA SW] Menjalankan background sync telemetri tertunda...');
        event.waitUntil(
            // Beri sinyal ke tab aktif untuk flush queue
            self.clients.matchAll().then((clients) => {
                clients.forEach((client) => {
                    client.postMessage({ type: 'SYNC_OFFLINE_TELEMETRY' });
                });
            })
        );
    }
});
