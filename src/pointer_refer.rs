pub fn run(){
    //primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values of primitive array: {:?}", (arr1, arr2));

    //With non-primitves, if you assing another variable to a piece of data, the first variable will no longer hold that value.
    //You'll need to use a reference (&) to point to the resource.
    // non-primitves
    //vector (vector is a non-primitive value)
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values of Vec: {:?}", (&vec1, vec2));
}
