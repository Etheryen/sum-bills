package parsing

import (
	"os"
	"slices"
	"strconv"
	"strings"
)

func GetCurrency(billLine string) string {
	var currencyBuffer string

	for _, char := range billLine {
		if char == ' ' {
			break
		}
		if !isDigit(char) && !isAllowedSymbol(char) {
			currencyBuffer += string(char)
		}
	}

	return currencyBuffer
}

func GetBillsSum(billLines []string) (float64, error) {
	var sum float64

	for _, line := range billLines {
		var billBuffer string

		for _, char := range line {
			if isDigit(char) || isAllowedSymbol(char) {
				billBuffer += string(char)
				continue
			}
			break
		}

		bill, err := strconv.ParseFloat(billBuffer, 64)
		if err != nil {
			return 0, err
		}

		sum += bill
	}

	return sum, nil
}

var ALLOWED_SYMBOLS = []rune{'+', '-', '.'}

func isAllowedSymbol(char rune) bool {
	return slices.Contains(ALLOWED_SYMBOLS, char)
}

func isDigit(char rune) bool {
	_, err := strconv.Atoi(string(char))
	return err == nil
}

func GetBillLines(header string, filePath string) ([]string, error) {
	file, err := os.ReadFile(filePath)
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(file), "\n")

	var billLines []string
	headerFound := false

	for _, line := range lines {
		if line == header {
			headerFound = true
			continue
		}
		if headerFound && line != "" {
			billLines = append(billLines, line)
		}
	}

	return billLines, nil
}
