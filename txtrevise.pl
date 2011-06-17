#!/usr/bin/env perl

#
# Txtrevise
# Command line text editing tool
# Version 1.1
# Copyright (c) 2009, 2011 Sam Saint-Pettersen
#
# Released under the MIT License
#
# Ported from Python to Perl.
#
use strict;
use Getopt::Long;
use English;

my $version = '1.1'; # Means compatible with Python-based version 1.1.
my $notverbose;

sub main {
	##
	# Main method.
	##
	# Produce verbose output by default.
	$notverbose = 0;

	# Count arguments provided, if none;
    # display "No arguments.." message and usage.
    if(@ARGV < 1) {
	    displayError("No options specified")
	}

	# Process provided arguments
    my $filename = "";
    my $match;
    my $repl;
    my $lineno = 1;
    my $help = 0;

    Getopt::Long::GetOptions(
    	'h' => \$help,
    	'f=s' => \$filename,
    	'l=i' => \$lineno,
    	'm=s' => \$match,
    	'r=s' => \$repl,
    	'q' => \$notverbose
	);

	if($help == 1) {
		displayUsage();
	}
    
    # With necessary argument, process file.
	if( $filename =~ /\w+/) {
        processFile($filename, $lineno, $match, $repl)
	}
}

sub processFile {
	##
    # Process file.
    # $_[0]: File to read/write.
    # $_[1]: Line number to read.
    # $_[2]: Word(s) to look for.
    # $_[3]: Replacement word(s) for match(es).
    ##
    my $linenum = 0;
    my $index = 0;
    my @lines = [];
    my $selline;
    my $file;
    # Read each line in file sequentially, store selected line no.
    # +<: open for reading and writing.
    open(FILE, "+<$_[0]") || displayError("Could not open specified file");
    while(<FILE>) {
        $lines[$linenum] = $_;
        if($linenum == $_[1] - 1) { # - 1, because lines start at 0.
            $selline = $_;
            $index = $linenum;
        }
        $linenum++;
    }

    # Revise the selected line.
    $lines[$index] = matchReplace($selline, $_[1], $_[2], $_[3]);
    seek(FILE, 0, 0); # Go to beginning of file to overwrite contents.
    foreach(@lines) {
        print FILE $_;
    }
    close(FILE);
}

sub matchReplace {
    ##
    # Match and replace word(s).
    # $_[0]: Line with matched word(s) to replace.
    # $_[1]: Line number to to do match and replace on.
    # $_[2]: Word(s) to match.
    # $_[3]: Word(s) to replace match(es) with.
    # return: Edited line.
    ##
    my $newline = $_[0];
    # If word(s) are matched, return edited line with replacement word(s).
    if($_[0] =~ m/($_[2])/) {
        if($notverbose == 0) {
            print "\nMatched at Line $_[1]: $_[0]";
        }
        $newline =~ s/($_[2])/$_[3]/;
        if($notverbose == 0) {
            print "\nReplaced with: $newline\n";
        }
    }
    # Otherwise, return same line as before.
    else {
        if($notverbose == 0) {
            print "\nNo matches at Line $_[1].\n"
        }
    }
    return $newline;
}

sub displayUsage {
	##
    # Display usage information.
    ##
    print "\nTxtrevise v $version ($OSNAME)\n";
    print "Command line text editing tool\n";
    print "Copyright (c) 2009, 2011 Sam Saint-Pettersen\n";
    print "\nReleased under the MIT License\n";
    print "\nPorted from Python to Perl\n";
    print "\nUsage: txtrevise.pl [-h] (-q) -f <file> -l <line #> -m <word(s)>\n";
    print "\t-r <word(s)>\n";
    print "\n\t-f: File to edit\n";
    print "\t-l: Line number to edit text on (starts at 1)\n";
    print "\t-m: Word(s) to match\n";
    print "\t-r: Replacement word(s) for matched word(s)\n";
    print "\t-q: Quiet mode. Only output to console for errors\n";
    print "\t-h: This help information\n\n";
    exit;
}

sub displayError {
	##
	# Display an error message and usage instructions.
	# $_[0]: Error to display in error message.
	##
	print "\nError: $_[0].\n";
	displayUsage();	
}

# Invoke main method.
main();
