/*
🧪 Practice 1: Simple Closure
✅ Task:
Write a closure that multiplies two numbers and prints the result.
*/
// fn main() {
//     let multiply = |x, y| x * y;
//     let result = multiply(2, 2);
//     println!("Product = {}", result);
// }

/*
🧪 Practice 2: Closure Capturing Environment
✅ Task:
Create a closure that prints a greeting using a name variable from the environment.
*/
// fn main() {
//     let name = String::from("Alice");
//     let greeting = || println!("Hello, {}", name);
//     greeting();
// }

/*
🧪 Practice 3: Passing a Closure to a Function
✅ Task:
Create a function called run that takes a closure and calls it. Then pass a closure to it from main.
*/
// fn run<F>(action: F)
// where
//     F: Fn(),
// {
//     action()
// }

// fn main() {
//     let greeting = || println!("Hi from closue");
//     run(greeting);
// }

/*
🧪 Practice 3: Passing a Closure to a Function
✅ Task:
Create a function called run that takes a closure and calls it. Then pass a closure to it from main.
*/
fn apply_fn<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    let square = |x| x * x;
    let result = apply_fn(square, 2);
    println!("Result: {}", result)
}
