#!/usr/bin/perl
package CPMScriptInterface;

use strict;
use warnings;

use Cache::FileCache;
use FindBin;
use File::Spec;
use Cwd;

my $shell_scrip_location = File::Spec->catdir( $main::CoreDir, 'ShellScripts' );
my $PS_exe_symlinker =
  File::Spec->canonpath("$shell_scrip_location\\SymlinkExe.ps1");
my $cache_location = File::Spec->catdir( $main::CoreDir, 'Cache' );

sub shell_script_location {
    if ( -e $PS_exe_symlinker ) {
        if ( -f $PS_exe_symlinker ) {
            return;
        }
        else {
            die "SymlinkExe.ps1 is not a file: $PS_exe_symlinker\n";
        }
    }
    else {
        die "SymlinkExe.ps1 does not exist: $PS_exe_symlinker\n";
    }
}

sub execute_symlink {
    ( my $exe_name, my $new_exe_location, my $symlink_exe_location ) = @_;

    my $command =
"PowerShell -NoProfile -ExecutionPolicy Bypass -File \"$PS_exe_symlinker\" -linkName \"$exe_name\" -destination \"$new_exe_location\" -target \"$symlink_exe_location\"";
    print "Executing: $command\n";

    my $exit_status = system($command);
    if ( $exit_status == 0 ) {
        print "Successfully executed SymlinkExe.ps1";
    }
    else {
        die "Failed to execute SymlinkExe.ps1: $!\n";
    }
}

1;
