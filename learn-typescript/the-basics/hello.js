"use strict";
console.log("Hello world!");
/*
// Bad code that will give an error because function call doesn't have enough arguments
function greet(person, date) {
    console.log(`Hello ${person}, today is ${date}!`);
}

greet("Brendan");
*/
// Remember to call `tsc --noEmitOnError <filename>` instead of just `tsc <filename>`
// to compile typescript file to javascript
function greet(person, date) {
    console.log(`Hello ${person}, today is ${date.toDateString()}!`);
}
greet("Brendan", new Date());
