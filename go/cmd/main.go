package main

import (
	"flag"
	"fmt"
	"os"
	"sum-bills/flags"
	"sum-bills/parsing"
)

func main() {
	header, filePath := flags.GetAll()

	if header == "" || filePath == "" {
		flag.Usage()
		os.Exit(1)
	}

	billLines, err := parsing.GetBillLines(header, filePath)
	if err != nil {
		fmt.Println("Error parsing lines:", err)
		os.Exit(1)
	}

	if len(billLines) == 0 {
		fmt.Println("No bills found")
		return
	}

	sum, err := parsing.GetBillsSum(billLines)
	if err != nil {
		fmt.Println("Error parsing bills:", err)
		os.Exit(1)
	}

	currency := parsing.GetCurrency(billLines[0])

	fmt.Printf("Sum is %v%v\n", sum, currency)
}
