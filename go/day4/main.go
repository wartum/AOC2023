package main

import (
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

const INPUT_NAME = "input.txt"

type Card struct {
	id          int
	win_numbers [10]int
	my_numbers  [25]int
}

var cards_n_cache map[int]int = make(map[int]int)
var cards = make([]int, 0, 20)

func (card *Card) CalculateN() int {
	n := 0
	for _, v := range card.win_numbers {
		if slices.Contains(card.my_numbers[:], v) {
			n += 1
		}
	}
	return n
}

func CalculatePoints(cardId int) int {
	n := cards_n_cache[cardId]
	points := 0
	for range n {
		if points == 0 {
			points = 1
		} else {
			points *= 2
		}
	}
	return points
}

func GetNextCards(cardId int) []int {
	n := cards_n_cache[cardId]
	cards := cards[:0]
	for i := range n {
		next_card_id := i + cardId + 1
		if next_card_id <= 218 {
			cards = append(cards, next_card_id)
		}
	}
	return cards
}

func getInput() []string {
	bytes, err := os.ReadFile(INPUT_NAME)
	if err != nil {
		log.Fatalf("Cannot read %v. %v", INPUT_NAME, err)
	}

	var builder strings.Builder
	builder.Write(bytes)
	return strings.Split(builder.String(), "\n")
}

func getCards(input []string) []Card {
	cards := make([]Card, 0, len(input))
	for _, line := range input {
		if line == "" {
			continue
		}
		colon_split := strings.Split(line, ":")

		// Retrieve Id
		id_str := strings.TrimSpace(strings.TrimPrefix(colon_split[0], "Card"))
		id, err := strconv.Atoi(id_str)
		if err != nil {
			log.Fatalln(err)
		}

		pipe_split := strings.Split(colon_split[1], "|")

		// Retrieve winning numbers
		var win_numbers [10]int
		i := 0
		for _, v := range strings.Split(pipe_split[0], " ") {
			val, err := strconv.Atoi(v)
			if err != nil {
				continue
			}
			win_numbers[i] = val
			i += 1
		}

		// Retrieve my numbers
		var my_numbers [25]int
		i = 0
		for _, v := range strings.Split(pipe_split[1], " ") {
			val, err := strconv.Atoi(v)
			if err != nil {
				continue
			}
			my_numbers[i] = val
			i += 1
		}

		cards = append(cards, Card{
			id:          id,
			win_numbers: win_numbers,
			my_numbers:  my_numbers,
		})
	}
	return cards
}

func main() {
	input := getInput()
	original_deck := getCards(input)
	for _, card := range original_deck {
		cards_n_cache[card.id] = card.CalculateN()
	}

	sum := 0
	for _, card := range original_deck {
		sum += CalculatePoints(card.id)
	}
	fmt.Println(sum)

	stack1 := make([]int, 0, len(original_deck))
	stack2 := make([]int, 0, len(stack1))
	for _, card := range original_deck {
		stack1 = append(stack1, card.id)
	}
	finish := false
	for !finish {
		for _, cardId := range stack1 {
			stack2 = slices.Concat(stack2, GetNextCards(cardId))
		}
		fmt.Println(len(stack2))

		if len(stack2) == 0 {
			finish = true
		} else {
			tmp := stack1
			stack1 = stack2
			stack2 = tmp[:0]
		}
	}

	fmt.Println(len(stack1))
}
