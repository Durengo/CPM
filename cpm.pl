#!/usr/bin/perl
use strict;
use warnings;
use File::Spec;
use Getopt::Long;
use Cwd;

my $os = $^O;

if ( $os eq 'MSWin32' ) {
    print "Windows platform detected.\n";
}
elsif ( $os eq 'linux' ) {
    die "Linux is not supported.\n";
}
elsif ( $os eq 'darwin' ) {
    die "Mac OS X is not supported.\n";
}
else {
    die "Unknown operating system: $os\n";
}

my $this_dir = getcwd();
