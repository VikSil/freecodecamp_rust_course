// const generics are useful in certain situations, but they are rarely used

struct Array<
    T,
    const N: usize // both T and N re generic types
> {
    data: [T; N], // here T is the type of array elements and N is the number of elements
    // the use of generic const N: usize makes Array<i32,3> and Array<i32,4> different types
}

fn main() {
    let array: [Array<i32, 3>; 3] = [
        // array of structs of type array
        Array {
            data: [1, 2, 3],
        },
        Array {
            //data: [4.0.,5.0,6.0], // this would not work because array variable holds Array<i32,3> type variables
            data: [4, 5, 6],
        },
        Array {
            // data: [7,8] // this would not work because array variable holds Array<i32,3> type variables
            data: [7, 8, 9],
        },
    ];

    println!("Success!");

    let floats: [Array<f64, 2>; 3] = [
        Array {
            data: [2.0, 3.4],
        },
        Array {
            data: [6.7, 5.6],
        },
        Array {
            data: [3.4, 5.6],
        },
    ];

    println!("Success!");
}
