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
use warnings;
use Getopt::Long;
use English;

my $version = '1.1'; # Means compatible with Python-based version 1.1.
my $verbose;

sub main {
	##
	# Main method.
	##
	# Produce verbose output by default.
	$verbose = 1;

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
    	'q!' => \$verbose
	);

	if($help == 1) {
		displayUsage();
	}
	if(@ARGV > 2) {
		if($filename != "") {
			processFile($filename, $lineno, $match, $repl)
		}
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
    my $selline = '';
    open(FILE, ">>$_[0]");
    foreach(@lines) {
    	print "$_\n";
    }  
    close(FILE);
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
