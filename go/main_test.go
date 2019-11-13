package main

import (
	"sync"
	"testing"
)

func benchmarkFib(fibN uint, b *testing.B) {
	b.ReportAllocs()
	var wg sync.WaitGroup
	for i := 2; i < b.N; i++ {
		wg.Add(1)
		go FibonacciBig(fibN, &wg)
	}
	wg.Wait()
}

func BenchmarkFib2k(b *testing.B)  { benchmarkFib(2000, b) }
func BenchmarkFib3k(b *testing.B)  { benchmarkFib(3000, b) }
func BenchmarkFib10k(b *testing.B) { benchmarkFib(10000, b) }
func BenchmarkFib20k(b *testing.B) { benchmarkFib(20000, b) }
func BenchmarkFib40k(b *testing.B) { benchmarkFib(40000, b) }
func BenchmarkFib50k(b *testing.B) { benchmarkFib(50000, b) }

func benchmarkFibSeq(fibN uint, b *testing.B) {
	b.ReportAllocs()
	for i := 2; i < b.N; i++ {
		var wg sync.WaitGroup
		wg.Add(1)
		FibonacciBig(fibN, &wg)
	}
}

func BenchmarkFibSeq2k(b *testing.B)  { benchmarkFibSeq(2000, b) }
func BenchmarkFibSeq3k(b *testing.B)  { benchmarkFibSeq(3000, b) }
func BenchmarkFibSeq10k(b *testing.B) { benchmarkFibSeq(10000, b) }
func BenchmarkFibSeq20k(b *testing.B) { benchmarkFibSeq(20000, b) }
func BenchmarkFibSeq40k(b *testing.B) { benchmarkFibSeq(40000, b) }
