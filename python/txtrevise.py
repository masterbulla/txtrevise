#!/usr/bin/env python

"""
Txtrevise
Command line text editing tool
Version 1.2
Copyright (c) 2009, 2014 Sam Saint-Pettersen

Released under the MIT License
"""
import sys
import getopt
import re
import zipimport # Just so cx_Freeze works.

version = '1.2' # Application version

def main():
    """
    Main method
    """
    # Produce verbose output by default
    global verbose; verbose = True
    
    # Count arguments provided, if none;
    # display "No arguments.." message and usage
    if len(sys.argv) < 2:
        displayError("No options specified")

    # Process provided arguments
    filename = match = repl = "(none)"
    lineNo = 1
    try:
        # Command line structure:
        # -h -f <filename> -l <line #> -m <word(s)> -r <word(s)>
        switches, args = getopt.getopt(sys.argv[1:], "hqf:l:m:r:")

        # When args are valid, perform appropriate action
        for s, a in switches:
            if s == "-h": displayUsage() # With switch "-h", display usage
            if s == "-f": filename = a # With switch "-f", specify filename
            if s == "-l": lineNo = int(a) # With switch "-l", specify line no.
            if s == "-m": match = a # With switch "-m", specify match word(s)
            if s == "-r": repl = a # With switch "-r", specify replacement
            if s == "-q": verbose = False # With switch "-q", suppress output

        # With necessary arguments, process file
        if len(sys.argv) > 2:
            if filename != "":
                processFile(filename, lineNo, match, repl)

    # On exception(s), display an error message and usage
    except getopt.GetoptError, ex:
        displayError(ex)

    except ValueError:
        displayError("Line number must be an integer")

def processFile(filename, lineNo, match, repl):
    """
    Process file
    @param filename File to read/write
    @param lineNo Line number to read
    @param match Word(s) to look for
    @param repl Replacement word(s) for match(es)
    """
    lineNum = index = 0
    allLines = []
    selLine = ""
    try:
        # Read each line in file sequentially, store selected line no
        f = open(filename, "r+") # r+: open for reading and writing
        for line in f:
            allLines.append(line)
            if lineNum == lineNo - 1: # - 1, because lines start at 0
                selLine = line
                index = lineNum
            lineNum += 1
                
        # Revise the selected line
        allLines[index] = matchReplace(selLine, lineNo, match, repl)
        f.seek(0) # Go to beginning of file to overwrite contents
        f.writelines(allLines) # Write all lines out to file
        f.close() # Close file

    # On I/O related exception, display an error message and usage
    except IOError:
        displayError("Invalid filename or permissions")
        
def matchReplace(line, lineNo, match, repl):
    """
    Match and replace word(s)
    @param line Line with matched word(s) to replace
    @param lineNo Line number to to do match and replace on
    @param match Word(s) to match
    @param repl Word(s) to replace match(es) with
    @return newLine Edited line
    """
    # If word(s) are matched, return edited line with replacement word(s)
    if re.search(match, line):    
        if verbose: print("\nMatched at Line %d: %s" % (lineNo, line))
        newLine = re.sub(match, repl, line)
        if verbose: print("Replaced with: %s" % newLine)

    # Otherwise, return same line as before
    else:
        if verbose: print("\nNo matches at Line %d." % lineNo)
        newLine = line

    return newLine

def displayUsage():
    """
    Display usage information
    """
    print("\nTxtrevise v %s (%s)" % (version, sys.platform))
    print("Command line text editing tool")
    print("Copyright (c) 2009, 2014 Sam Saint-Pettersen")
    print("\nReleased under the MIT License")
    print("\nUsage: %s [-h] (-q) -f <file> -l <line #> -m <word(s)>" % sys.argv[0])
    print("\t-r <word(s)>")
    print("\n\t-f: File to edit")
    print("\t-l: Line number to edit text on (starts at 1)")
    print("\t-m: Word(s) to match")
    print("\t-r: Replacement word(s) for matched word(s)")
    print("\t-q: Quiet mode. Only output to console for errors")
    print("\t-h: This help information")
    sys.exit(0)

def displayError(err):
    """
    Display an error message and usage instructions
    @param err Error to display in error message
    """
    print("\nError: %s." % err)
    displayUsage()

# Invoke main method
if __name__ == "__main__": main()
