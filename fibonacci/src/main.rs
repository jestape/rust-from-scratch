fn main() {

    let n = 10;

    let mut term_2 = 1;
    let mut term_1 = 1;
    
    for _number in 3..n+1 {
        let term_0 = term_1;
        term_1 = term_2 + term_1;
        term_2 = term_0;   
    }

    println!("{} is the {}th fibonacci number", term_1, n);

}
