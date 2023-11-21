use strict;
use warnings;

use Term::Menus;

my $selection = &pick(["Option 1", "Option 2", "Exit"], '', '> ', 'AUTO');
print "You picked: $selection\n";