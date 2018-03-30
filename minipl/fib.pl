
var prev2 : int := 0;
var prev1 : int := 1;

print prev2;
print ", ";
print prev1;

var i : int;

for i in 0 .. 25 do
    var next: int := prev1 + prev2;
    print ", ";
    print next;
    prev2 := prev1;
    prev1 := next;
end for;
