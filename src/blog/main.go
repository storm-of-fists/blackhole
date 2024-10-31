package main

import (
	"fmt"
	"log"
	"net/http"
	"text/template"
)

var COUNTER = 0

// https://youtu.be/TT7SV-bAZyA
func main() {
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		template := template.Must(template.ParseFiles("./static/index.html"))
		template.Execute(w, nil)
	})

	http.HandleFunc("/posts", func(w http.ResponseWriter, r *http.Request) {
		template := template.Must(template.ParseFiles("./templates/post-1/index.html"))
		template.Execute(w, nil)
	})

	http.HandleFunc("/counter", func(w http.ResponseWriter, r *http.Request) {
		template := template.Must(template.New("counter").Parse(fmt.Sprintf("<div id=\"counter\">%d</div>", COUNTER)))
		template.Execute(w, nil)
	})

	http.HandleFunc("/decrease", func(w http.ResponseWriter, r *http.Request) {
		COUNTER -= 1
		w.Header().Add("Hx-Trigger", "counterChange")
	})

	http.HandleFunc("/increase", func(w http.ResponseWriter, r *http.Request) {
		COUNTER += 1
		w.Header().Add("Hx-Trigger", "counterChange")
	})

	log.Println("App running on 8000")
	log.Fatal(http.ListenAndServe(":8000", nil))
}

// type Post
