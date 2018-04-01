print "Give a number: "; 
var n : int;
read n;
var v : int := 1;
var i : int;
for i in 1..n do 
    v := v * i;
end for;
print "The result is: ";
print v;
