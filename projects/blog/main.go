package main

import (
	"log"
	"net/http"
	"text/template"
)
// https://youtu.be/TT7SV-bAZyA
func main() {
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		template := template.Must(template.ParseFiles("./templates/index.html"))
		template.Execute(w, nil)
	})

	http.HandleFunc("/search", func(w http.ResponseWriter, r *http.Request) {
		template := template.Must(template.ParseFiles("./templates/post-1/index.html"))
		// data := map[string][]Stock{
		// 	"Results": SearchTicket(r.URL.Query.Get("key"))
		// }
		template.Execute(w, nil)
	})

	log.Println("App running on 8000")
	log.Fatal(http.ListenAndServe(":8000", nil))
}

// type Post
