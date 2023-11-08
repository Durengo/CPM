#!/usr/bin/perl
package CPMBuildInterface;

use strict;
use warnings;

use Cache::FileCache;
use FindBin;
use File::Spec;
use Cwd;

# my $build_py_location = File::Spec->catdir( $FindBin::Bin, '..', 'PyScripts' );
# my $build_py          = File::Spec->canonpath("$build_py_location/build.py");
# my $cache_location    = File::Spec->catdir( $FindBin::Bin, '..', 'Cache' );

my $build_py_location = File::Spec->catdir( $main::CoreDir, 'PyScripts' );
my $build_py       = File::Spec->canonpath("$build_py_location\\build.py");
my $cache_location = File::Spec->catdir( $main::CoreDir, 'Cache' );

sub check_build_py {
    if ( -e $build_py ) {
        if ( -f $build_py ) {

            # print "build_py_location: $build_py_location\n";
            # print "build_py: $build_py\n";
            return;
        }
        else {
            die "build.py is not a file: $build_py\n";
        }
    }
    else {
        die "build.py does not exist: $build_py\n";
    }
}

sub clear_build_cache {
    my $file_remove = "options_cache.json";

    my $working_directory = getcwd();
    chdir $cache_location
      or die "Unable to change directory: $cache_location\n";

    my $full_path = File::Spec->catfile( $cache_location, $file_remove );

    if ( -e $file_remove ) {
        unlink $file_remove or die "Unable to remove $file_remove: $!";
        print "File $full_path removed successfully.\n";
    }
    else {
        print "File $full_path does not exist.\n";
    }

    chdir $working_directory
      or die "Unable to change directory: $working_directory\n";
}

sub get_build_py {
    return $build_py;
}

sub get_script_location {
    return $build_py_location;
}

1;
