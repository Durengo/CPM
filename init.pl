#!/usr/bin/perl
use strict;
use warnings;
use FindBin;
use File::Spec;
use Getopt::Long;
use Cwd;

my $os;
my $this_dir;
my $provided_dir;

main();

sub main {
    check_os();
    setup();
    create_and_write_cpm_file();
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

sub setup {
    $this_dir = $FindBin::Bin;
    $this_dir =~ s/\//\\/g;
    $provided_dir = $ARGV[0];

    print("this_dir: $this_dir\n");
    print("provided_dir: $provided_dir\n");

    if ( $this_dir eq $provided_dir ) {
        die "The provided directory is the same as the current directory.\n";
    }

    chdir $this_dir
      or die "Unable to change directory: $this_dir\n";
}

sub create_and_write_cpm_file {
    my $cpm_script     = File::Spec->catfile( $this_dir,     'cpm.pl' );
    my $new_cpm_script = File::Spec->catfile( $provided_dir, 'cpm.pl' );
    print("cpm_script: $cpm_script\n");
    print("cpm_script: $new_cpm_script\n");

    open( my $source_fh, '<', $cpm_script )
      or die "Could not open file 'cpm.pl' in '$this_dir' $!";
    open( my $dest_fh, '>', $new_cpm_script )
      or die "Could not open file 'cpm.pl' $!";

    while ( my $line = <$source_fh> ) {
        print $dest_fh $line;
    }

    close $source_fh;
    close $dest_fh;

    if ( $os eq 'linux' ) {
        chmod 0755, $new_cpm_script;
    }
}
