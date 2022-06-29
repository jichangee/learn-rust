fn main() {
    fn6();
}

fn fn6() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}

// fn fn5() {
//     let a = [1, 2, 3, 4, 5];

//     for element in a {
//         println!("the value is: {}", element);
//     }
// }

// fn fn4() {
//     let mut number = 3;

//     while number !=0 {
//         println!("{}!", number);
//         number -= 1;
//     }
// }

// fn fn3() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;

//         if counter == 11 {
//             break if counter == 10 { counter * 2 } else { counter * 3};
//         }
//     };

//     println!("The result is {}", result);
// }

// fn fn1() {
//     loop {
//         println!("Hello, world!");
//     }
// }

// fn fn2() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {}", count);
// }