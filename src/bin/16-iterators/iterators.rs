fn main() {

    // using for loop

    let nums = vec![1,2,3];
    for value in nums {  //ownership of nums passes over here to avoid that we have more iterators
        println!("{}",value);
    }




    // Iterating after creating an `iterator`
    /* The iter() method in Rust provides a way to iterate over the elements of a collection by borrowing them. 
    You can’t mutate the variables since we have an immutable reference to the internal elements */

    let nums1 = vec![1,2,3]; // this is vector the type
    let iter = nums.iter(); //this is iterator the type is 
    for val in iter {
        println!("{}",val);
    }




    // Iterating using `.next`
    /* You can’t mutate the data here either The `iterator` is mutable, 
    but the inner elements (val) still is an immutable reference */

    let nums2 = vec![1,2,3];
    let iter2 = nums.iter();
    while let Some(val) = iter2.next() {
        println!("{}", val);
    }




    // IterMut
    let mut nums3 = vec![1,2,3];
    let iter3 = nums.iter_mut();
    for val in iter3 {
        *val = *val + 1;
    }
    println!("{}",val);




    // IntoIter
    /* The IntoIterator trait is used to convert a collection into an iterator that takes ownership of the collection.
    Useful when
    1. You no longer need the original collection
    2. When you need to squeeze performance benefits by transferring ownership (avoiding references) */

    let nums4 = vec![1,2,3];
    let iter4 = nums.into_iter();
    for val in iter4 {
        println!("{}", val);
    }




    // Consuming Adaptors
    let v1 = vec![1,2,3];
    let itt = v1.iter(); 
    let total = itt.sum();
    // let total2 = itt.sum(); // this cant be done because the iterator cant be used again




    // Iterator Adaptors 
    let x1 = vec![1,2,3];
    let iter_x1 = x1.iter();
    let iter_x2 = iter_x1.map(|x| x + 1); 
    for x in iter_x2 {
        println!("{}", x);
    }

    // filter
    let z1 = vec![1,2,3];
    let iter_z1 = z1.iter();
    let iter_z2 = iter_z1.filter(|x| *x % 2 == 0);
    for x in iter_z2 {
        println!("{}", x);
    }




    // lill assigment
    fn test_ag(v: Vec<i32>) -> Vec<i32> {
        let new_v = v.iter().filter(|x| *x % 2 == 1).map(|x| x + 1);
        let res: Vec<i32> = new_v.collect();
        res
    }
}