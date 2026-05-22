// HTTP server that sets the headers needed for SharedArrayBuffer.

package main

import (
	"flag"
	"log"
	"net/http"
)

func main() {
	addr := flag.String("addr", "localhost:8080", "address to listen on")
	dir := flag.String("dir", ".", "directory to serve")
	flag.Parse()

	files := http.FileServer(http.Dir(*dir))
	handler := withHeaders(files)

	log.Printf("serving %s at http://%s", *dir, *addr)
	log.Fatal(http.ListenAndServe(*addr, handler))
}

func withHeaders(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		h := w.Header()

		// Cross-origin isolation: required for SharedArrayBuffer in browsers.
		h.Set("Cross-Origin-Opener-Policy", "same-origin")
		h.Set("Cross-Origin-Embedder-Policy", "require-corp")

		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusNoContent)
			return
		}

		next.ServeHTTP(w, r)
	})
}
