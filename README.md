# Amortization Calculator
I often find the online amortization (am-er-tuh-zey-shuhn) schedule calculators lacking. Thus, I am creating my own and designing it to include the features I wish others had. Why is it written in Rust? Just wanted to learn the language!
<br><br>
What will this calculator offer that others do not?
  1. variable additional monthly payments (allowing me to plan for increases or decreases over time)
  2. variable additional yearly payments (think tax refund and months with three paychecks)
  3. multiple one-time payments
  4. a separate table that shows me what additional monthly payments would do to total interest paid and life of the loan
<br><br>

## Future plans
  - calculate optimized additional payment amount
  - compare additional payments on current loan with refi to determine which is the better option
<br><br>
## Learning schedule and lessons learned
_(This is mostly just so I can come back and reference things later)_
#### Day 1
1. mapped out entire project - what I want it to look like and what user input fields would be needed
2. got really excited to write the first couple lines and realized I didn't even know how to get user input
3. started reading [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) - Foreward, Intro, Chapter 1
    - [Interactive Experimental version of the book](https://rust-book.cs.brown.edu/) - I highly recommend!

#### Day 2 <br>
1. started reading Chapter 3 of The Rust Programming Language (being new to programming, I opted to read this chapter first so I would better understand the concepts behind the game code being written in chapter 2)

#### Day 3
1. finished Chapter 3
2. listened to [How To Think Like A Programmer](https://youtu.be/azcrPFhaY9k?)
    * followed process from video and wrote out each line I thought I would need in English (commented-out form) for the amortization calculator

#### Day 4  
1. read Chapter 2
    * created the game as I went
2. started writing rust for each line I had written the day before
3. looked into how to get user input and assign to a variable<br>
    ````
    use std::io;
    
    fn main() {
    
        println!("Prompt for user input goes here");
    
        let mut var = String::new();
        io::stdin().read_line( &mut var ).expect("Failed to read entry");
    
    }
    ````
breakdown:
  * `use std::io;` goes at the beginning of the code, before the main function. This loads the input/output (io) module from the standard (std) Rust library so that functions can use them later
  - the following 3 lines go in the main function
  - `println!("text")` is a macro that will print anything quoted in parenthesis as output for the user to see and then start a new line (this is different from the print!() macro)
  - `let mut var` defines a mutable variable named "var", mutable indicating that the value assigned to the variable can be changed. By default, Rust variables are immutable.
  - `= String::new()` finishes that line by assigning a new string value to "var"
  - `io::stdin()` calls to the "standard input" trait(?) of the io module. This goes back to the first line we wrote, "use std::io"
  - `.read_line (&mut var)` indicates that the user input should be read to (think assigned to) to mutable variable "var"
  - `.expect("text")` tells the program what to print to the end user if an error occurs at that point. The other arm(?) is "Ok"
<br>

4\. looked into changing string variable "var" to a number (float specifically as we'll be dealing with decimals) <br>

    ````
    let var: f32 = var.trim().parse().expect("Must be a number");
    ````
breakdown
   - `let var: f32` creates a new immutable variable "var" that will be a 32-bit float (aka a decimal number)
   - `= var` refers to the mutable variable "var" from user input and assigns it to the new IMMUTABLE "var" - pretty sure you can change the name here so it's not "var" and "var", but my brain likes it that way so I have fewer variables sitting around
  - `.trim()` tells rust to trim anything (spaces, non-float characters, \n, etc) from both the front and back of the user input string assigned to "var"
  - `.parse()` is a str method that parses the string type into another type [reference](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
  - `.expect("text")` prints "text" to the user if the program errors at this point <br>
<br>

5\. learned about writing calculations as code <br>
  - for basic math, you can use the following: <br>
    - multiplication: * <br>
      ````
      let x = 2 * 2;
      println!("x is equal to {x}");
      ````
      > output: x is equal to 4  <br>
    - division: / <br>
      ````
      let x = 6 / 2;
      println!("x is equal to {x}");
      ````
      > output: x is equal to 3  <br>
    - addition: +
      ````
      let x = 6 + 2;
      println!("x is equal to {x}");
      ````
      > output: x is equal to 8  <br>
    - subtraction: -
      ````
      let x = 6 - 2;
      println!("x is equal to {x}");
      ````
      > output: x is equal to 4  <br>
    - modulo (remainder): %
      ````
      let x = 6 % 2;
      println!("x is equal to {x}");
      ````
      > output: x is equal to 0  <br>
 - for a number to the power of another number (so like 2^4 = 16), you would write:
   ````
   let x = pow(2, 4);
   println!("x is equal to {x}");
   ````
   > output: x is equal to 16  <br>
 - for a float to the power of a number (so 2.2^2, or 2.2 squared), you would write:
   ````
   let x = f32::powf(2.2, 2.0);
   println!("x is equal to {x}");
   ````
   > output: x is equal to 4.84  <br>
     - notice here that there are a few differences from the previous example:
       - the type `f32` is indicated after the equal sign (=) but before the power indication
       - to indicate a power with a float, `powf` is used rather than `pow` ("pow" would be used for signed integers types (`i32`) and unsigned integer types (`u32`))
       - because we are working with a float type, rust requires that all numbers in the calculation be in decimal form. as such, `let x = f32::powf(2.2, 2)` would **not** work because the "2" at the end is an integer and not a float

#### Day 5  
1. experimenting with `for`, `while`, and `if`
    - **goal:** write a block of code that will count the duration (number of months) remaining for the life of the loan. this will be used to calculate the new principal balance each month along with the interest paid and payment made toward the principal amount
    - **initial thought:** use `while` where `t_num` is the user input for total months of loan converted from a string to i32 (see Day 4 items 3 and 4 for syntax)
      ````
      ...
      let m_array = [1 .. t_num];
      let mut index = t_num - 1;
      while index >= 0 {
          println!("month is {}", m_array[index]);
          index -= 1;
      }
      ````
      - issues:
        - as is, the block will not compile due to the first line, `let m_array = [1 .. t_num];`
          - without using a module, an array cannot use the syntactical shorthand for "between 1 and t_num". changing the first two lines to the below will work
            ````
            let m_array = [1, 2, 3, 4, 5];
            let mut index = 5 - 1;
            ````
        - the resulting program will panic with error "attempt to subtract with overflow", which is not ideal
          - this is due to the second to last line, `index -=1`
    - **using for:**
      ````
      ...
      let m_arr = [1 .. t_num];
      for index in m_arr {
          println!("the month is {index}");
      }
      ````
      - issues:
        - we run into the same issue as using `while` where the first line of code does not work without incorporating a module
        - this will increase from 1 - t_num rather than counting down from t_num to 1 (needs to be descending for other calculations and to track the total number of months paying on the loan)
      - improvements compared to `while`:
        - looks more clean
        - does not cause program to panic
    - **using for (better):**
      ````
      ...
      for month in (1..=t_num).rev() {
          println!("{month}");
      }
      println!("this works!");
      ````
      - improvements:
        - it works with t_num :)
        - it does not panic
        - it counts backwards (descending) from t_num to 1
      - breakdown:
        - `for month in (1..=t_num)` this is a for loop with variable "month" being assigned a value in the range from 1 to t_num.
          - Note that "=t_num" indicates that t_num should be included in the range. If it were written `(1..t_num)` then the range would be from 1 to t_num - 1
        - `.rev()` indicates that the value assigned to "month" will be in reverse order in the range, meaning that the first value assigned will be t_num, then t_num - 1, then t_num -2, etc. until the final value of 1 is reached
        - `{` this indicates the beginning of the code block that the for loop will perform
        - `println!("{month}");` prints the value assigned to month for each iteration of the for loop (from t_num to 1)
        - `}` ends the for loop
        - `println!("this works!");` once the for loop exits, the program will print "this works!" to the screen
       
#### Day 6
After taking a multi-month break to learn other things, I finally had time to circle back and work on this project again. During my time off, I completed a Udemy Python course which I think was both a blessing and a curse. It helped me understand what I needed to do and how to do it, but also I kept trying to write everything in Python and had to keep googling so I could "translate" to Rust. <br>
<br>
Accomplishments: <br>
<br>
1. Successfully built the base "while" loop for the project<br>
2. Successfully wrote the main equations and tested them (so much testing!) <br>
<br>
Sadly, I didn't use any of the code from 'Day 5', but it will still make a great reference for future projects and there was still value in the testing done <br>
<br>
Next steps: <br>
<br>
1. Calculate the month that principal payment and interest payment flip (when you're officially paying more toward principal than interest each month)<br>
2. Incorporate calculations for additional regular payments<br>
3. Incorporate calculations for additional irregular payments<br>
4. Table output<br>
5. Add column or two to recalc total interest paid if you put an extra $50 in monthly (and then allow user input for a second amt)<br>
6. Compare additional payments on current loan with refi to determine which is the better option (include disclaimer that this is not official financial advice lol)<br>

#### Day 7<br>

Focus: writing the code for additional monthly payments of a regular amount (this turned out to be A LOT harder than I thought). I was trying to do something like this:<br>
````
- Ask user: "Do you want to incorporate regular additional monthly payments? y or n "
- Retrieve user input, assign to $choice
- If $choice is "y", ask user "How much extra do you want to pay?" and assign to $additional_monthly_payment
- If $choice is "n", output "You chose not to make regular additional monthly payments."
- If $choice is neither "y" nor "n", output "you must type either y or n", then ask again
````
Seems simple and fairly straightforward, right? Whelp, what I didn't account for (or honestly even know) is that Rust identifies characters and strings as two different structures. So my code that looked like this
````
use std::io;
fn main() {
    println!("pick one (y or n)");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read entry");
    println!("You chose {}",choice);
    if choice == "y" {
        println!("you picked yes");
    } else if choice == "n" {
        println!("you picked no");
    } else {
        println!("error");}
}
````
just. kept. failing. After lots of googling, testing in the [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021), and millions of interruptions from children (naps, snacks, play, dance parties, etc.), I had determined that my user input was coming in as a string type but that my "if else" statement was comparing that string to "y" or "n" which Rust identified as a character. After some more playing with no success in getting the above code to run, I finally decided to make that a "feature request" (lol) and stick with asking the users for number input (1 is "yes" and 0 is "no). With that, my code ran beautifully. Below is a simplified version of what I wrote:
````
use std::io;
fn main() {
    println!("pick one (0 is no, 1 is yes)");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read entry");
        let choice: i32 = choice.trim().parse().expect("Try again");    
    //println!("You chose {}",choice);
    if choice == 1 {
        println!("you picked yes");
    } else if choice == 0 {
        println!("you picked no");
    } else {
        println!("error");}
}
````
