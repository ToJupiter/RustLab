pub fn shadowing () {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x inside this is: {}", x);
    }

    println!("The real value of x in the shadowing scope is: {}", x);
}