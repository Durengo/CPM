#!/usr/bin/perl
use strict;
use warnings;
use FindBin;
use File::Spec;
use File::Copy;

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
    win_create_and_write_cpm_file();
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

sub win_create_and_write_cpm_file {
    print("Creating '$provided_dir\\cpm.pl'.\n");

    unless ( -d "$this_dir\\Cache" ) {
        mkdir "$this_dir\\Cache"
          or die "Unable to create directory: '$this_dir\\Cache'\n";
    }

    my $cpm_script =
      File::Spec->catfile( $this_dir, 'Presets', 'Perl', 'cpm_win' );

    if ( -e "$this_dir\\Cache\\cpm_win" ) {
        unlink "$this_dir\\Cache\\cpm_win"
          or die "Failed to delete '$this_dir\\Cache\\cpm_win': $!";
    }

    my $cached_cpm_script =
      File::Spec->catfile( $this_dir, 'Cache', 'cpm_win' );

    copy( $cpm_script, $cached_cpm_script )
      or die "Unable to copy file: $cpm_script\n";

    open( my $fh, '<', $cached_cpm_script )
      or die "Unable to open file: $cached_cpm_script\n";

    my $file_contents = do { local $/; <$fh> };

    close $fh;

    my $old_string = '# my $core_dir = this;';

    my $temp_this_dir = $this_dir;
    $temp_this_dir =~ s/\\/\\\\/g;
    my $new_string = '$core_dir = "' . $temp_this_dir . '";';

    $file_contents =~ s/\Q$old_string\E/$new_string/;

    open( $fh, '>', $cached_cpm_script )
      or die "Unable to open file: $cached_cpm_script\n";

    print $fh $file_contents;

    close $fh;

    if ( -e "$provided_dir\\cpm.pl" ) {
        unlink "$provided_dir\\cpm.pl"
          or die "Failed to delete '$provided_dir\\cpm.pl': $!";
    }

    my $new_cpm_script = File::Spec->catfile( $provided_dir, 'cpm.pl' );

    open( my $source_fh, '<', $cached_cpm_script )
      or die "Could not open file 'cpm.pl' in '$this_dir' $!";
    open( my $dest_fh, '>', $new_cpm_script )
      or die "Could not open file 'cpm.pl' $!";

    while ( my $line = <$source_fh> ) {
        print $dest_fh $line;
    }

    close $source_fh;
    close $dest_fh;
}

# Keepin as template for other OSes.
sub global_create_and_write_cpm_file {
    print("Creating '$provided_dir\\cpm.pl'.\n");

    my $cpm_script     = File::Spec->catfile( $this_dir,     'cpm.pl' );
    my $new_cpm_script = File::Spec->catfile( $provided_dir, 'cpm.pl' );

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
