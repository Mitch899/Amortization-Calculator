# AmortizationCalculator
This amortization (am-er-tuh-zey-shuhn) calculator is designed to build on the features you find with most
online amotization schedule calculators. When playing with my mortgage and calculating early payments, I 
often found them lacking. 
<br><br>
What does this calculator offer that others don't?
  1. variable additional monthly payments (allowing you to plan for increases or decreases over time)
  2. variable additional yearly payments (think tax refund and months with three paychecks)
  3. multiple one-time payments
  4. table that shows you what additional monthly payments would do to your interest paid and life of your loan
<br><br>

## Future plans:
  - calculate optimized additional payment amount
  - compare additional payments on current loan with refi to determine which is the better option
<br><br>
## Lessons learned from this project

#### Day 1
- mapped out entire project - what I want it to look like and what user input fields would be needed
- got really excited to write the first couple lines and realized I didn't even know how to get user input
- started reading The Rust Programming Language (https://doc.rust-lang.org/book/title-page.html)
  - Foreward, Intro, Chapter 1
<br><br>
#### Day 2 <br>
- started reading Chapter 3 of The Rust Programming Language (being new to programming, I opted to read this chapter first so I would better understand the concepts behind the game code being written in chapter 2)
<br><br>
#### Day 3
- finished Chapter 3
- listened to "How To Think Like A Programmer" (https://youtu.be/azcrPFhaY9k?)
  - followed process from video and wrote out each line I thought I would need in English (commented-out form) for the amortization calculator
<br><br>
#### Day 4  
- read Chapter 2 and created the game as I went
- started writing rust for each line I had written the day before
- looked into how to get user input and assign to a variable
> use std::io<br>
>> println!("Prompt for user input goes here");<br>
>> let mut var = String::new(); ..creates new mutable string variable "var"<br>
>> io::stdin().read_line( &mut var ).expect("Failed to read entry");<br>
  - "use std::io" goes at the beginning of the code, before the main function. This loads the input/output (io) module from the standard (std) Rust library so that functions can use them later
  - the following 3 lines go in the main function
  - "println!("text")" is a macro that will print anything quoted in parenthesis as output for the user to see and then start a new line (this is different from the print!() macro)
  - "let mut var" defines a mutable variable named "var", mutable indicating that the value assigned to the variable can be changed. By default, Rust variables are immutable.
  - "= String::new()" finishes that line by assigning a new string value to "var"
  - "io::stdin()" calls to the "standard input" trait(?) of the io module. This goes back to the first line we wrote, "use std::io"
  - ".read_line (&mut var)" indicates that the user input should be read to (think assigned to) to mutable variable "var"
  - ".expect("text")" tells the program what to print to the end user if an error occurs at that point. The other arm(?) is "Ok"
- looked into changing string variable "var" to a number (float specifically as we'll be dealing with decimals)
> let var: f32 = var.trim().parse().expect("Must be a number");
  - "let var: f32" creates a new immutable variable "var" that will be a 32-byte float (aka a decimal number)
  - "= var" refers to the mutable variable "var" from user input and assigns it to the new IMMUTABLE "var" - pretty sure you can change the name here so it's not "var" and "var", but my brain likes it that way so I have fewer variables sitting around
  - ".trim()" tells rust to trim anything (spaces, non-float characters, \n, etc) from both the front and back of the user input string assigned to "var"
  - ".parse()" NO CLUE WHAT THIS DOES
  - ".expect("text") prints "text" to the user if the program errors at this point
