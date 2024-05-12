package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func IsDigit(r rune) bool {
	return r >= '1' && r <= '9'
}

type Game struct {
	Id    int
	Cubes map[string]int
}

func ParseInput() []Game {
	bytes, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("Couldn't read file. %v", err)
	}
	games := make([]Game, 0)
	b := strings.Builder{}
	b.Write(bytes)
	lines := strings.Split(b.String(), "\n")
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		game := Game{
			Id:    0,
			Cubes: make(map[string]int),
		}
		words := strings.Split(line, " ")
		for i := 0; i < len(words); {
			switch {
			case words[i] == "Game":
				id, _ := strings.CutSuffix(words[i+1], ":")
				game.Id, _ = strconv.Atoi(id)
				i += 2
			case IsDigit(rune(words[i][0])):
				val, _ := strconv.Atoi(words[i])
				color := words[i+1]
				color, _ = strings.CutSuffix(color, ",")
				color, _ = strings.CutSuffix(color, ";")
				if game.Cubes[color] < val {
					game.Cubes[color] = val
				}
				i += 2
			default:
				i += 1
			}
		}
		games = append(games, game)
		fmt.Println(game)
	}
	return games
}

func main() {
	sum, power := 0, 0
	games := ParseInput()
	for _, game := range games {
		if game.Cubes["red"] <= 12 && game.Cubes["green"] <= 13 && game.Cubes["blue"] <= 14 {
			sum += game.Id
		}
		power += game.Cubes["red"] * game.Cubes["green"] * game.Cubes["blue"]
	}
	fmt.Printf("%v %v\n", sum, power)
}
