package main

import (
	"fmt"
	"math/big"
	"sync"
)

// FibonacciBig calculates Fibonacci number using bit.Int
func FibonacciBig(n uint, wg *sync.WaitGroup) *big.Int {
	defer wg.Done()
	var n2, n1 = big.NewInt(0), big.NewInt(1)
	for i := uint(1); i < n; i++ {
		n2.Add(n2, n1)
		n1, n2 = n2, n1
	}
	fmt.Print(".")
	return n1
}

func fibAsync(n int) {
	var wg sync.WaitGroup
	for i := 1000; i < n; n++ {
		wg.Add(1)
		go FibonacciBig(uint(i), &wg)
	}
	wg.Wait()
}
