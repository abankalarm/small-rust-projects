important commands

let bytes= s.as_byte
for (i,&item) in bytes.iter().enumerate(){
    if item==b' '{
        return i;
