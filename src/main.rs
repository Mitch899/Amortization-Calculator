//https://www.rocketmortgage.com/learn/how-to-calculate-mortgage
//https://www.bankrate.com/loans/personal-loans/how-to-calculate-loan-interest/
use std::io;

fn main () {
    //ASK user for LOAN PRINCIPAL AMOUNT
    println!("What was your original principal loan amount?");
    //save input to variable
    let mut p = String::new();
    io::stdin()
        .read_line(&mut p)
        .expect("Failed to read entry");
        let mut p: f32 = p.trim().parse().expect("Must be a number");
    println!("Your principal is {p}");
    let initial_principal = p;

    //ASK user for INTEREST RATE
    println!("What is your interest rate? (ex. 5 or 7.2)");
    //save input to variable
    let mut i = String::new();
    io::stdin()
        .read_line(&mut i)
        .expect("Failed to read entry");
        let i: f32 = i.trim().parse().expect("Must be a number");
    //calculate: divide i by 100 (annual interest) and then divide again by 12 to get your monthly interest
    let ii: f32 = (i/100.0)/12.0;
    println!("Your interest rate is {ii}");

    //ASK user for LOAN TERM
    println!("How many months is your loan term?");
    //save input to variable
    let mut loan_term = String::new();
    io::stdin()
        .read_line(&mut loan_term)
        .expect("Failed to read entry");
    //MOVE to calculation for new p
    let loan_term_float: f32 = loan_term.trim().parse().expect("Must be a number");
    //KEEP HERE
    let loan_term_num: i32 = loan_term.trim().parse().expect("Needs to be a number");
    println!("Your loan term is {loan_term_num} months");



    let mut idx: f32 = 1.0;
    let mut total_interest: f32 = 0.0;
    //let mut balance: f32 = p;

    while idx <= loan_term_float {
            println!("Month {idx}");
            idx += 1.0;
            // EQUATIONS

            //println!("line 52 is {p}");
            // monthly payments
            // M = P ((r(1+r)^n) / (((1+r)^n)-1))
            // m = p ((ii(1+ii)^loan_term_float) / (((1+ii)^loan_term_float)-1))
            // m = p ( (ii(b)) / (b-1) )
            let a = 1.0 + ii;
            let b = f32::powf(a, loan_term_float);
            let m: f32 = initial_principal *(ii*b)/(b-1.0);
            println!("Monthly payment is {m}. This does not include escrow fees.");

            //println!("line 71 is {p}");
            // interest paid this month
            let interest_payment: f32 = p *ii;
            println!("Interest paid this month is {interest_payment}");

            // monthly principal
            let principal_paid: f32 = m - interest_payment;
            println!("Amount paid toward principal is {principal_paid}");

            //println!("line 76 is {p}");
            //new principal
            p -= principal_paid;
            println!("New principal is {p}");

            total_interest += interest_payment;
            println!("Total interst paid to date is {total_interest}");

            println!("---------------");

            //Identify which month you pay more toward principal than interest
            //if principal_paid - interest_payment > 2.0 {
            //    print!("You will now be paying more toward principal monthly!");
            //} else {
            //    print!("");
            //}
        }



//notes for later:
//tables - the long way https://stackoverflow.com/questions/30379341/how-to-print-well-formatted-tables-to-the-console
//tables - the pretty way https://github.com/phsym/prettytable-rs
}
