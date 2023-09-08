package main

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"

	_ "github.com/lib/pq"
)

// DB Settings
const (
	host     = "192.168.0.107"
	port     = 5432
	user     = "postgres"
	password = "Worldwarcraft3!@"
	dbname   = "postgres"
)

func main() {

	// DB
	psqlconn := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable", host, port, user, password, dbname)
	db, err := sql.Open("postgres", psqlconn)
	if err != nil {
		panic(err)
	}
	err = db.Ping()
	if err != nil {
		panic(err)
	}
	fmt.Println("Connected to the database...")

	// HTTP Server
	fileServer := http.FileServer(http.Dir("./html"))
	http.Handle("/", fileServer)
	fmt.Printf("Starting the HTTP server on port 8000\n")
	if err := http.ListenAndServe(":8000", nil); err != nil {
		log.Fatal(err)
	}

}
