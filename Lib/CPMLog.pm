#!/usr/bin/perl
package CPMLog;

use strict;
use warnings;

my $RED   = "\e[31m";
my $GREEN = "\e[32m";
my $RESET = "\e[0m";

sub _print_col {
    my ( $color, $text ) = @_;
    print "$color$text$RESET\n";
}

sub trace {
    print "@_\n";
}

sub error {
    _print_col( $RED, @_ );
}

sub info {
    _print_col( $GREEN, @_ );
}

1;
