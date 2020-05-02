package main

import (
	"fmt"
	"testing"
)

func TestGetStringForNumber(t *testing.T) {
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
			actual := GetStringForNumber(testItem.input)
			if actual != testItem.expected {
				t.Errorf("Got %s, expected %s", actual, testItem.expected)
			}
		})
	}
}
