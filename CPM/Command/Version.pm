#!/usr/bin/perl
use strict;
use warnings;

package CPM::Command::Version;
use base qw(CLI::Framework::Command);

my $version = "0.0.1";

sub run {
    my $self = shift;
    my $opts = shift;

    return "CPM Version: $version";
}
