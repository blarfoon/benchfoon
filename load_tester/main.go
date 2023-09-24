package main

import (
	"fmt"
	"math/rand"
	"net/http"
	"os"
	"strconv"
	"time"

	vegeta "github.com/tsenart/vegeta/v12/lib"
)

func main() {
	rand.Seed(time.Now().UnixNano())
	ip := os.Args[1]
	port := os.Args[2]
	frequency, err := strconv.Atoi(os.Args[3])
	if err != nil {
		panic(err)
	}
	size, err := strconv.Atoi(os.Args[4])
	if err != nil {
		panic(err)
	}

	var users string
	for i := 0; i < size; i++ {
		users = users + fmt.Sprintf("user_list=user%d", i)
		if i != size-1 {
			users = users + "&"
		}
	}

	url := fmt.Sprintf("http://%s:%s/json_serialize?%s", ip, port, users)

	rate := vegeta.Rate{Freq: frequency, Per: time.Second}
	duration := 20 * time.Second
	targeter := vegeta.NewStaticTargeter(vegeta.Target{
		Method: "GET",
		URL:    url,
		Header: http.Header{
			"Content-Type": []string{"application/json"},
		},
	})
	attacker := vegeta.NewAttacker()

	var metrics vegeta.Metrics
	for res := range attacker.Attack(targeter, rate, duration, "Big Bang!") {
		metrics.Add(res)
	}
	metrics.Close()

	// prints the report in stdout
	// total requests | p99 latency | req/s
	fmt.Printf("%d | %s | %f", metrics.Requests, metrics.Latencies.P99, metrics.Throughput)
}
