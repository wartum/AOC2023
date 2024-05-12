package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Point struct {
	X int
	Y int
}

type Symbol struct {
	IsGear         bool
	ConnectedParts []int
	Points         []Point
}

func NewSymbol(x, y int, isGear bool) Symbol {
	return Symbol{
		IsGear:         isGear,
		ConnectedParts: make([]int, 0),
		Points: []Point{
			{X: x - 1, Y: y - 1},
			{X: x, Y: y - 1},
			{X: x + 1, Y: y - 1},

			{X: x - 1, Y: y},
			{X: x + 1, Y: y},

			{X: x - 1, Y: y + 1},
			{X: x, Y: y + 1},
			{X: x + 1, Y: y + 1},
		},
	}
}

func IsDigit(r rune) bool {
	return r >= '0' && r <= '9'
}

func IsSymbol(r rune) bool {
	return !IsDigit(r) && r != '.'
}

func ReadFile() []string {
	bytes, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("Could not read input. %v", err)
	}
	b := strings.Builder{}
	b.Write(bytes)
	content := b.String()
	return strings.Split(content, "\n")
}

func main() {
	symbols := make([]Symbol, 200)
	x, y := 0, 0
	lines := ReadFile()
	for _, line := range lines {
		x = 0
		y += 1
		for _, r := range line {
			x += 1
			if IsSymbol(r) {
				symbols = append(symbols, NewSymbol(x, y, r == '*'))
			}
		}
	}

	x, y, sum := 0, 0, 0
	for _, line := range lines {
		strbld := strings.Builder{}
		isPart := false
		var currentSymbol *Symbol = nil
		y += 1
		x = 0
		for _, r := range line {
			x += 1
			if IsDigit(r) {
				strbld.WriteRune(r)
				if !isPart {
					for i, s := range symbols {
						for _, p := range s.Points {
							if p.X == x && p.Y == y {
								isPart = true
								currentSymbol = &symbols[i]
							}
						}
					}
				}
			} else if strbld.Len() > 0 {
				if isPart {
					digit, _ := strconv.Atoi(strbld.String())
					sum += digit
					if currentSymbol.IsGear {
						currentSymbol.ConnectedParts = append(currentSymbol.ConnectedParts, digit)
					}
				}
				isPart = false
				currentSymbol = nil
				strbld = strings.Builder{}
			}
		}

		if isPart {
			digit, _ := strconv.Atoi(strbld.String())
			sum += digit
			if currentSymbol.IsGear {
				currentSymbol.ConnectedParts = append(currentSymbol.ConnectedParts, digit)
			}
		}
	}

	ratios := 0
	for _, s := range symbols {
		if s.IsGear && len(s.ConnectedParts) == 2 {
			ratios += s.ConnectedParts[0] * s.ConnectedParts[1]
		}
	}

	fmt.Printf("Sum = %v, Ratios = %v\n", sum, ratios)
}
