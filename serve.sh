python3 -c "
import http.server, socketserver, mimetypes

mimetypes.add_type('application/wasm', '.wasm')
mimetypes.add_type('application/javascript', '.js')

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory='web', **kwargs)
    def end_headers(self):
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        super().end_headers()

with socketserver.TCPServer(('', 8080), Handler) as httpd:
    print('Serving at http://localhost:8080')
    httpd.serve_forever()
"