// Print statement!
print "Hello, world!\n";

// A few assignments
var a : int := 10;
var b : int := 20;

/*
|  \ /  |/ _ (_   _) | | \  ___)
|   v   | |_| || | | |_| |\ \   
| |\_/| |  _  || | |  _  | > >  
| |   | | | | || | | | | |/ /__ 
|_|   |_|_| |_||_| |_| |_/_____)
*/

var sumSquared : int := (a + b) * (a + b);

// A few assdertions
assert sumSquared = (b + a) * (b + a);
assert sumSquared = 900;

// And print something
print sumSquared;
