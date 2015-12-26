/* 
    Txtrevise (go)
    Command line text editing tool
    Version 1.0.0 (equivalent to Python impl. 1.1)
    Copyright (c) 2015 Sam Saint-Pettersen
    Released under the MIT License.
*/

package main

import ( 
    "bufio"
    "fmt"
    "os"
    "regexp"
    "strconv"
    "strings"
    "io/ioutil"
)

// Display error and then usage information.
func displayError(err string) {
	fmt.Println(fmt.Sprintf("Error: %s.", err))
	displayUsage()
    os.Exit(-1)
}

// Display usage information.
func displayUsage() {
	fmt.Println("\nTxtrevise v 1.0.0 (go)");
    fmt.Println("Command line text editing tool");
    fmt.Println("Copyright (c) 2015 Sam Saint-Pettersen");
    fmt.Println("\nReleased under the MIT License");
    fmt.Println(fmt.Sprintf("\nUsage: %s [-h] (-q) -f <file> -l <line #> -m <word(s)>", os.Args[0]));
    fmt.Println("-r <word(s)>");
    fmt.Println("\n-f: File to edit");
    fmt.Println("-l: Line number to edit text on (starts at 1)");
    fmt.Println("-m: Word(s) to match");
    fmt.Println("-r: Replacement word(s) for matched word(s)");
    fmt.Println("-q: Quiet mode. Only output to console for errors");
    fmt.Println("-h: This help information");
}

// Process file.
func processFile(filename string, lineNo int, matches string, repl string, verbose bool) {
    var lineNum int = 0
	var index int = 0 
	var allLines []string

	// Read each line in file sequentually, store selected line no.
    file, err := os.Open(filename);
    if err != nil {
        displayError(fmt.Sprintf("Problem opening file: %s", filename))
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        allLines = append(allLines, scanner.Text())
        if lineNum == lineNo - 1 {
            index = lineNum
        }
        lineNum++
    }

    // Revise the selected line and append newline.
    allLines[index] = matchReplace(allLines[index], lineNo, matches, repl, verbose)
    allLines = append(allLines, "")

    // Write out changed lines to file.
    ioutil.WriteFile(filename, []byte(strings.Join(allLines, "\n")), 0644)
}

// Match and replace word(s).
func matchReplace(line string, lineNo int, matches string, repl string, verbose bool) (string) {
    var newLine string
    re, err := regexp.Compile(matches)

    if err != nil {
        displayError("Problem with regular expression")
    } 

    // If word(s) are matched, return edited line with replacement word(s).
    if re.MatchString(line) == true {
        if verbose {
            fmt.Println(fmt.Sprintf("\nMatched at Line %d: %s", lineNo, line))
        }
        newLine = re.ReplaceAllString(line, repl)
        if verbose {
            fmt.Println(fmt.Sprintf("Replaced with: %s", newLine))
        }
    } else {
        if verbose {
            fmt.Println(fmt.Sprintf("\nNo matches at Line %d", lineNo))
        }
        // Otherwise, return same line as before.
        newLine = line
    }
    return newLine
}

func main() {
    filename := ""
    matches := ""
    repl := ""
    verbose := true
    lineNo := 1

    if len(os.Args) > 1 {
        for i, a := range os.Args {
            if a == "-h" {
                displayUsage();
            }
            if a == "-f" {
                filename = os.Args[i+1]
            }
            if a == "-l" {
                var err error
                lineNo, err = strconv.Atoi(os.Args[i+1])
                if err != nil {
                    displayError("Line number must be an integer")
                }
                if lineNo == 0 {
                    displayError("Line number must be greater than 0")
                }
            }
            if a == "-m" {
                matches = os.Args[i+1]
            }
            if a == "-r" {
                repl = os.Args[i+1]
            }
            if a == "-q" {
                verbose = false;
            }
            i += 1
        }

    } else {
        displayError("No options specified")
    }

    // With necessary arguments, process file.
    if len(os.Args) > 2 && len(filename) > 0 {
	   processFile(filename, lineNo, matches, repl, verbose)
    }
}
