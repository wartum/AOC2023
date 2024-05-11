package main

import (
	"testing"
)

func TestIsDigit(t *testing.T) {
	digits := []rune{'1', '2', '3', '4', '5', '6', '7', '8', '9'}
	nonDigits := []rune{'a', 'y', '!'}

	for _, v := range digits {
		if !IsDigit(v) {
			t.Fatalf("'%c' is actually a digit", v)
		}
	}

	for _, v := range nonDigits {
		if IsDigit(v) {
			t.Fatalf("'%c' is actually not a digit", v)
		}
	}
}

func TestToDigit(t *testing.T) {
	data := map[rune]int32{
		'1': 1,
		'2': 2,
		'3': 3,
		'4': 4,
		'5': 5,
		'6': 6,
		'7': 7,
		'8': 8,
		'9': 9,
	}

	for k, v := range data {
		if ToDigit(k) != v {
			t.Fatalf("'%c' has been mapped to %v", k, ToDigit(k))
		}
	}

}

func TestCombine(t *testing.T) {
	type Data struct {
		a int32
		b int32
		c int32
	}
	data := []Data{
		{a: 5, b: 2, c: 52},
		{a: 9, b: 9, c: 99},
		{a: 1, b: 2, c: 12},
		{a: 3, b: 8, c: 38},
		{a: 8, b: 1, c: 81},
	}

	for _, v := range data {
		if v.c != Combine(v.a, v.b) {
			t.Fatalf("Combination error: '%v' + '%v' = %v", v.a, v.b, Combine(v.a, v.b))
		}
	}
}
