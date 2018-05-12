print "Give a number: "; 
var n : int;
read n;
var v : int := 1;
var i : int;
for i in 1..n {
    v := v * i;
}
print "The result is: ";
print v;
