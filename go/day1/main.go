package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

const INPUT_NAME = "input.txt"

var DIGITS_MAP = map[string]int32{
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

func IsDigit(r rune) bool {
	return r >= '1' && r <= '9'
}

func ToDigit(r rune) int32 {
	return r - '0'
}

func Combine(a int32, b int32) int32 {
	return a*10 + b
}

type LineAnalyzer struct {
	last_rune rune
	strbld    strings.Builder
}

func (analyzer *LineAnalyzer) AddRune(r rune) {
	analyzer.strbld.WriteRune(r)
	analyzer.last_rune = r
}

func (analyzer *LineAnalyzer) Analyze() int32 {
	if IsDigit(analyzer.last_rune) {
		return ToDigit(analyzer.last_rune)
	}

	for k, v := range DIGITS_MAP {
		if strings.Contains(analyzer.strbld.String(), k) {
			analyzer.strbld.Reset()
			analyzer.strbld.WriteRune(analyzer.last_rune)
			return v
		}
	}

	return -1
}

func readInput() []string {
	raw, err := os.ReadFile(INPUT_NAME)
	if err != nil {
		log.Fatalf("Could not read file. %v", err)
	}

	builder := strings.Builder{}
	builder.Write(raw)
	content := builder.String()
	return strings.Split(content, "\n")
}

func main() {
	var sum int32 = 0
	lines := readInput()
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}

		var a, b int32 = -1, -1
		var analyzer LineAnalyzer
		for _, r := range line {
			analyzer.AddRune(r)
			ret := analyzer.Analyze()
			if ret != -1 {
				if a == -1 {
					a = ret
				} else {
					b = ret
				}
			}
		}

		if b == -1 {
			b = a
		}

		sum += Combine(a, b)
	}

	fmt.Printf("%v\n", sum)
}
