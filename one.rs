fn main()
let mut a = "play with rust";
let b = "something else";

a=b;  # passing over the ownership

println!("{}", a);

let c = a; # error

