use std::collections::HashMap;
fn main() {

// ------------------------type alias------------------------
// use `type` to create a new type alias
// type Foo = Bar;
type Miles = u64;
type Centimeters = u64;
type Callbacks = HashMap<String, Box<dyn Fn(i32, i32) -> i32>>;
struct Contact {
name: String,
phone: String,
}
type ContactName = String;
// type aliases can contain other type aliases
type ContactIndex = HashMap<ContactName, Contact>;
// type aliases can be used anywhere a type can be used
fn add_contact(index: &mut ContactIndex, contact: Contact) {
index.insert(contact.name.to_owned(), contact);
}
// type aliases can also contain lifetimes ...
type BorrowedItems<'a> = Vec<&'a str>;
// ... and also contain generic types
// type GenericThings<T> = Vec<Thing<T>>;

// ------------------------new types------------------------
// This block uses type aliases instead of New Types:
{
type Centimeters = f64;
type Kilograms = f64;
type Celsius = f64;
fn add_distance(a: Centimeters, b: Centimeters) -> Centimeters {
a + b
}
fn add_weight(a: Kilograms, b: Kilograms) -> Kilograms {
a + b
}
fn add_temperature(a: Celsius, b: Celsius) -> Celsius {
a + b
}
let length = 20.0;
let weight = 90.0;
let temp = 27.0;
// Since type aliases are the same as their underlying type,
// it's possible to accidentally use the wrong data as seen here:
let distance = add_distance(weight, 10.0);
let total_weight = add_weight(temp, 20.0);
let new_temp = add_temperature(length, 5.0);
}
// This block uses new types instead of type aliases:
{
// create 3 tuple structs as new types, each wrapping f64
struct Centimeters(f64);
struct Kilograms(f64);
struct Celsius(f64);
fn add_distance(a: Centimeters, b: Centimeters) -> Centimeters {
// access the field using .0
Centimeters(a.0 + b.0)
}
fn add_weight(a: Kilograms, b: Kilograms) -> Kilograms {
Kilograms(a.0 + b.0)
}
fn add_temperature(a: Celsius, b: Celsius) -> Celsius {
Celsius(a.0 + b.0)
}
// the type must be specified
let length = Centimeters(20.0);
let weight = Kilograms(90.0);
let temp = Celsius(27.0);
let distance = add_distance(length, Centimeters(10.0));
let total_weight = add_weight(weight, Kilograms(20.0));
let new_temp = add_temperature(temp, Celsius(5.0));
// using the wrong type is now a compiler error:
// let distance = add_distance(weight, Centimeters(10.0));
// let total_weight = add_weight(temp, Kilograms(20.0));
// let new_temp = add_temperature(length, Celsius(5.0));
}

}
