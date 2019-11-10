package main

import (
	"fmt"
	"math/big"
)

// FibonacciBig calculates Fibonacci number using bit.Int
func FibonacciBig(n uint) *big.Int {
	var n2, n1 = big.NewInt(0), big.NewInt(1)
	for i := uint(1); i < n; i++ {
		n2.Add(n2, n1)
		n1, n2 = n2, n1
	}
	return n1
}

func main() {
	fmt.Println("FIB(1000)=", FibonacciBig(1000))
}
