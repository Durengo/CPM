use strict;
use warnings;
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

my $working_dir = getcwd();
chdir $working_dir or die "Unable to change directory: $working_dir\n";

my %options;
my %option_actions = (
    "help"                    => sub { print_help(); },
    "debug"                   => sub { build_debug(); },
    "release"                 => sub { build_release(); },
    "clean"                   => sub { clean_nobuild(); },
    "clean-build"             => sub { clean_build(); },
    "clean-install"           => sub { clean_install(); },
    "clean-rebuild"           => sub { clean_rebuild(); },
    "clean-rebuild-release"   => sub { clean_nobuild(); build_release(); },
    "clean-reinstall"         => sub { clean_install_debug(); },
    "clean-reinstall-release" => sub { clean_install_release(); },
);

GetOptions( \%options, keys %option_actions )
  or die("Error in command line arguments.\n");

# print Dumper(\%options);

# Process options dynamically based on the option_actions hash
foreach my $option ( keys %options ) {
    if ( defined $options{$option} && exists $option_actions{$option} ) {
        $option_actions{$option}->( $options{$option} );
    }
}

# If no options provided, provide default behavior or instructions
unless ( keys %options ) {
    print "Building project in Debug for $os platform.\n";
    core_setup();
}

sub print_help {
    print "Usage:
    $0 [] - runs the build on the current platform and rebuilds everything in debug
    $0 [--help] - shows help
    $0 [--debug] - runs the build on the current platform and rebuilds everything in debug (does not clean)
    $0 [--release] - runs the build on the current platform and rebuilds everything in release (does not clean)
    $0 [--clean] - removes the \"build\" and the \"Install\" directories
    $0 [--clean-build] - removes the \"build\" directory
    $0 [--clean-install] - removes the \"Install\" directory
    $0 [--clean-rebuild] - removes the \"build\" and the \"Install\" directories and rebuilds everything in debug
    $0 [--clean-rebuild-release] - removes the \"build\" and the \"Install\" directories and rebuilds everything in release
    $0 [--clean-reinstall] - removes the \"Install\" directory and reinstalls in debug
    $0 [--clean-reinstall-release] - removes the \"Install\" directory and reinstalls in release
    ";
    exit;
}

sub clean_build {
    my $cmd2  = "git";
    my @args2 = ( "clean", "-dfx", "$working_dir/build" );

    my $exit_status2 = system( $cmd2, @args2 );
    if ( $exit_status2 == 0 ) {
        print "\"build\" cleaned successfully.\n";
    }
    else {
        die "Failed to clean \"build\": $!\n";
    }
}

sub clean_install {
    my $cmd2  = "git";
    my @args2 = ( "clean", "-dfx", "$working_dir/Install" );

    my $exit_status2 = system( $cmd2, @args2 );
    if ( $exit_status2 == 0 ) {
        print "\"Install\" cleaned successfully.\n";
    }
    else {
        die "Failed to clean \"Install\": $!\n";
    }
}

sub clean_install_debug {
    my @prerequisite_checks = (
        \&clean_install,
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pi", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
    );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}

sub clean_install_release {
    my @prerequisite_checks = (
        \&clean_install,
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pi", "Release" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
    );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}

sub clean_nobuild {
    my @prerequisite_checks = ( \&clean_build, \&clean_install, );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}

sub clean_rebuild {

    my @prerequisite_checks = (
        \&clean_build,
        \&clean_install,
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pg", "nt/msvc", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pb", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },

        # TODO: Breaks the cmake script. Need to fix.
        # Actually it works when the main file is not an exe(??)
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pi", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
    );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }

}

sub build_debug {
    my @prerequisite_checks = (
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pg", "nt/msvc", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pb", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },

        # TODO: Breaks the cmake script. Need to fix.
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pi", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
    );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}

sub build_release {
    my @prerequisite_checks = (
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pg", "nt/msvc", "Release" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pb", "Release" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },

        # TODO: Breaks the cmake script. Need to fix.
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pi", "Release" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
    );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}

sub core_setup {
    my @prerequisite_checks = (
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pg", "nt/msvc", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pb", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },

        # TODO: Breaks the cmake script. Need to fix.
        sub {
            chdir "utils" or die "Unable to change directory: utils\n";

            my $cmd  = "build.bat";
            my @args = ( "-pi", "Debug" );

            my $exit_status = system( $cmd, @args );
            if ( $exit_status == 0 ) {
                print "Project generated successfully.\n";
            }
            else {
                die "Failed to generate project: $!\n";
            }

            chdir $working_dir
              or die "Unable to change directory: $working_dir\n";
        },
    );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}
