package main

import (
	"io"
	"net/http"
)

func homeHTML(w http.ResponseWriter, r *http.Request) {
	io.WriteString(w, "Hello World!\n")
}

func main() {
	http.HandleFunc("/", homeHTML)
}
