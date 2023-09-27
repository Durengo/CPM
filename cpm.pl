#!/usr/bin/perl
use strict;
use warnings;
use FindBin;
use File::Spec;
use File::Copy;
use Getopt::Long;
use Cwd;

my $os;
my $this_dir;
my $provided_dir;

main();

sub main {
    if ( !defined $ARGV[0] ) {
        die "Please provide a directory to install cpm.pl\n";
    }
    print("Beginning initialization process.\n");
    os_specific();
    print("Finished initialization process.\n");
}

sub check_os {
    $os = $^O;
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
}

sub os_specific {
    check_os();
    my @steps;
    my $completed_steps = 0;
    my $total_steps     = 0;

    if ( $os eq 'MSWin32' ) {
        @steps = ( \&win_setup, \&for_windows, );

        $total_steps = scalar(@steps);

    }

    elsif ( $os eq 'linux' ) {
        for_linux();
    }

    elsif ( $os eq 'darwin' ) {
        for_mac();
    }

    foreach my $check_step (@steps) {
        $completed_steps++;
        print("[$completed_steps/$total_steps] ");
        $check_step->();
        print("\n");
    }
}

sub for_windows {
}

sub for_linux {

}

sub for_mac {

}

sub win_setup {
    print("Locating directories.\n");

    $this_dir = $FindBin::Bin;
    $this_dir =~ s/\//\\/g;
    $provided_dir = $ARGV[0];

    if ( $this_dir eq $provided_dir ) {
        die "The provided directory is the same as the current directory.\n";
    }

    chdir $this_dir
      or die "Unable to change directory: $this_dir\n";
}
