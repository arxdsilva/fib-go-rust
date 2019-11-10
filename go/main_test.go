package main

import "testing"

func BenchmarkFib1000(b *testing.B) {
	fibAsync(b.N)
}

// func BenchmarkFib2(b *testing.B)  { benchmarkFib1000(2000, b) }
// func BenchmarkFib3(b *testing.B)  { benchmarkFib1000(3000, b) }
// func BenchmarkFib10(b *testing.B) { benchmarkFib1000(10000, b) }
// func BenchmarkFib20(b *testing.B) { benchmarkFib1000(20000, b) }
// func BenchmarkFib40(b *testing.B) { benchmarkFib1000(40000, b) }
