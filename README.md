important commands

let bytes= s.as_byte
for (i,&item) in bytes.iter().enumerate(){
    if item==b' '{
        return i;
        

STRONG SLICES=>  let s = String::from("hello world");let hello = &s[0..5];

println!("rect1 is {:?}", rect1); . Putting the specifier :?
inside the curly brackets tells println! we want to use an output format called
Debug . The Debug trait enables us to print our struct in a way that is useful for
developers so we can see its value while we’re debugging our code.


VECTORS
let v = vec![100, 32, 57]; Creating a new vector containing values
let v: Vec<i32> = Vec::new(); Listing 8-1: Creating a new, empty vector to hold values of type i32
    
The type HashMap<K, V>
stores a mapping of keys of type K to values of type V . It does this via a hashing
function, which determines how it places these keys and values into memory.
Many programming languages support this kind of data structure, but they
often use a different name, such as hash, map, object, hash table, or
associative array, just to name a few.

generic in enum
Result<T, E> {
Ok(T),
Err(E),
}


generic in functon 
fn function_name<T>(variable_name: &<T>) -> T{}
