print "Hello, world!";

var a : int := 10;
var b : int := 20;
var sumSquared : int := (a + b) * (a + b);

assert sumSquared = (b + a) * (b + a);
assert sumSquared = 900;

print sumSquared;
