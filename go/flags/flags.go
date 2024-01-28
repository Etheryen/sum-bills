package flags

import "flag"

func GetAll() (string, string) {
	header := flag.String("header", "", "the line of text preceding bills")
	filePath := flag.String("file", "", "path of the file with bills")

	flag.Parse()

	return *header, *filePath
}
