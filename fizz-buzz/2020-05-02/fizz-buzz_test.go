package main

import (
	"fmt"
	"testing"
)

func TestGetStringForNumbers3And5(t *testing.T) {
	replacements := []IntReplacement{
		{
			divisor:     3,
			replacement: "Fizz"},
		{
			divisor:     5,
			replacement: "Buzz"},
	}

	var tests = []struct {
		input    int
		expected string
	}{
		{1, "1"},
		{2, "2"},
		{3, "Fizz"},
		{4, "4"},
		{5, "Buzz"},
		{6, "Fizz"},
		{7, "7"},
		{8, "8"},
		{9, "Fizz"},
		{10, "Buzz"},
		{11, "11"},
		{12, "Fizz"},
		{13, "13"},
		{14, "14"},
		{15, "FizzBuzz"},
		{16, "16"},
		{17, "17"},
		{18, "Fizz"},
		{19, "19"},
		{20, "Buzz"},
		{21, "Fizz"},
		{22, "22"},
		{23, "23"},
		{24, "Fizz"},
		{25, "Buzz"},
		{26, "26"},
		{27, "Fizz"},
		{28, "28"},
		{29, "29"},
		{30, "FizzBuzz"},
	}

	for _, testItem := range tests {
		testName := fmt.Sprintf("%d", testItem.input)
		t.Run(testName, func(t *testing.T) {
			actual := GetStringForNumber(testItem.input, replacements)
			if actual != testItem.expected {
				t.Errorf("Got %s, expected %s", actual, testItem.expected)
			}
		})
	}
}

func TestGetStringForNumbers6And7(t *testing.T) {
	replacements := []IntReplacement{
		{
			divisor:     6,
			replacement: "Bang"},
		{
			divisor:     7,
			replacement: "Boom"},
	}

	var tests = []struct {
		input    int
		expected string
	}{
		{5, "5"},
		{6, "Bang"},
		{7, "Boom"},
		{8, "8"},
		{9, "9"},
		{10, "10"},
		{11, "11"},
		{12, "Bang"},
		{13, "13"},
		{14, "Boom"},
		{15, "15"},
	}

	for _, testItem := range tests {
		testName := fmt.Sprintf("%d", testItem.input)
		t.Run(testName, func(t *testing.T) {
			actual := GetStringForNumber(testItem.input, replacements)
			if actual != testItem.expected {
				t.Errorf("Got %s, expected %s", actual, testItem.expected)
			}
		})
	}
}

func TestReplaceBetweenRange(t *testing.T) {
	replacements := []IntReplacement{
		{
			divisor:     3,
			replacement: "Fizz"},
		{
			divisor:     5,
			replacement: "Buzz"},
	}

	var tests = []struct {
		start, end int
		expected   string
	}{
		{1, 5, "1 2 Fizz 4 Buzz"},
		{3, 7, "Fizz 4 Buzz Fizz 7"},
		{88, 92, "88 89 FizzBuzz 91 92"},
	}

	for _, testItem := range tests {
		testName := fmt.Sprintf("%d, %d", testItem.start, testItem.end)
		t.Run(testName, func(t *testing.T) {
			actual := ReplaceBetweenRange(testItem.start, testItem.end, replacements, " ")
			if actual != testItem.expected {
				t.Errorf("Got %s, expected %s", actual, testItem.expected)
			}
		})
	}
}
