#!/usr/bin/perl
use strict;
use warnings;
use FindBin;
use File::Spec;
use File::Copy;

my $os;
my $this_dir;
my $provided_dir;

my $RED   = "\e[31m";
my $GREEN = "\e[32m";
my $RESET = "\e[0m";

main();

sub main {
    if ( !defined $ARGV[0] ) {
        die "Please provide a directory to install cpm.pl\n";
    }
    print_col( $GREEN, "Beginning initialization process.\n" );
    os_specific();
    print_col( $GREEN, "\nFinished initialization process." );
}

sub print_col {
    my ( $color, $text ) = @_;
    print "$color$text$RESET\n";
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
        @steps = (
            \&win_setup, \&global_init_cache,

            # Libraries
            # \&for_all_install_term_lib,
            # \&for_all_install_log4perl_lib,

            # Libraries
            \&win_copy_preset, \&win_create_new_cpm, \&win_create_bat_for_cpm,
        );

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
    }
}

sub for_windows {

}

sub for_linux {

}

sub for_mac {

}

# sub for_all_install_term_lib {
#     print("Checking if Term::Menus is installed.\n");
#     my $module = 'Term::Menus';

#     eval "require $module";

#     if ( !$@ ) {
#         print("Term::Menus is already installed.\n");
#         print_col( $GREEN, "[DONE]" );
#         return;
#     }
#     else {
#         print("Term::Menus is not installed.\n");
#     }

#     print("Installing Term::Menus.\n");
#     my $cmd  = "cpanm";
#     my @args = ("TMMemHandle");

#     print("Executing: $cmd @args\n");

#     my $exit_status = system( $cmd, @args );
#     if ( $exit_status == 0 ) {
#         print_col( $GREEN, "[DONE]" );
#         return;
#     }
#     else {
#         die "Failed to execute '$cmd', '@args': $!\n";
#     }
# }

# sub for_all_install_log4perl_lib {
#     print("Checking if Log::Log4perl::Level is installed.\n");
#     my $module = 'Log::Log4perl::Level';

#     eval "require $module";

#     if ( !$@ ) {
#         print("Log::Log4perl::Level is already installed.\n");
#         print_col( $GREEN, "[DONE]" );
#         return;
#     }
#     else {
#         print("Log::Log4perl::Level is not installed.\n");
#     }

#     print("Installing Log::Log4perl::Level.\n");
#     my $cmd  = "cpanm";
#     my @args = ("Log::Log4perl");

#     print("Executing: $cmd @args\n");

#     my $exit_status = system( $cmd, @args );
#     if ( $exit_status == 0 ) {
#         print_col( $GREEN, "[DONE]" );
#         return;
#     }
#     else {
#         die "Failed to execute '$cmd', '@args': $!\n";
#     }
# }

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

    print_col( $GREEN, "[DONE]" );
}

sub win_copy_preset {
    print("Copying preset.\n");

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
    my $new_string = 'my $core_dir = "' . $temp_this_dir . '";';

    $file_contents =~ s/\Q$old_string\E/$new_string/;

    open( $fh, '>', $cached_cpm_script )
      or die "Unable to open file: $cached_cpm_script\n";

    print $fh $file_contents;

    close $fh;

    print_col( $GREEN, "[DONE]" );
}

sub win_create_new_cpm {
    print("Creating new cpm.pl\n");

    if ( -e "$provided_dir\\cpm.pl" ) {
        unlink "$provided_dir\\cpm.pl"
          or die "Failed to delete '$provided_dir\\cpm.pl': $!";
    }

    my $new_cpm_script = File::Spec->catfile( $provided_dir, 'cpm.pl' );
    my $cached_cpm_script =
      File::Spec->catfile( $this_dir, 'Cache', 'cpm_win' );

    open( my $source_fh, '<', $cached_cpm_script )
      or die "Could not open file 'cpm.pl' in '$this_dir' $!";
    open( my $dest_fh, '>', $new_cpm_script )
      or die "Could not open file 'cpm.pl' $!";

    while ( my $line = <$source_fh> ) {
        print $dest_fh $line;
    }

    close $source_fh;
    close $dest_fh;

    print_col( $GREEN, "[DONE]" );
}

sub win_create_bat_for_cpm {
    print("Creating entrypoint for cpm.pl\n");

    my $batch_entrypoint = File::Spec->catfile( $provided_dir, 'cpm.bat' );

    open( my $fh, '>', $batch_entrypoint )
      or die "Could not open file 'cpm.bat' $!";

    print $fh "\@echo off\n";
    print $fh "perl cpm.pl %*\n";

    close $fh;

    print_col( $GREEN, "[DONE]" );
}

sub global_init_cache {
    print("Initializing cache.\n");

    unless ( -d "$this_dir\\Cache" ) {
        mkdir "$this_dir\\Cache"
          or die "Unable to create directory: '$this_dir\\Cache'\n";
    }

    print_col( $GREEN, "[DONE]" );
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
