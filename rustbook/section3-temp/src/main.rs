fn main() {

    // Fahrenheit to Celsius using the concept of shadowing, although it might not be the
    // best place to use it.

    let temperature = 14;
    let temperature = (temperature-32)*5/9; 
    println!("{} Celsius", temperature);
}
