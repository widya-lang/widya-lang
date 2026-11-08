#!/usr/bin/env python3
"""
Server Web Ringan untuk ArenaSilat.id
Platform Digital Pencak Silat Berdaulat Indonesia
"""

import http.server
import socketserver
import os
import sys

PORT = 8080
DIRECTORY = os.path.join(os.path.dirname(os.path.abspath(__file__)), "web")

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def end_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
        super().end_headers()

def run_server():
    port = PORT
    for attempt in range(5):
        try:
            with socketserver.TCPServer(("", port), Handler) as httpd:
                print("==================================================================")
                print(f"🥋 ARENASILAT.ID - SERVER WEB SUITE AKTIF PADA PORT: {port}")
                print(f"🌐 Akses Web Portal di : http://localhost:{port}")
                print(f"📂 Direktori Aset      : {DIRECTORY}")
                print("==================================================================")
                sys.stdout.flush()
                httpd.serve_forever()
        except OSError as e:
            if "Address already in use" in str(e):
                port += 1
            else:
                raise e

if __name__ == "__main__":
    run_server()
