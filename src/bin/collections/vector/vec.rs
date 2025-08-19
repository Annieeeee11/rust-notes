fn main() {
    let mut x = Vec::new(); // new vector
    x.push(1); //pushed values cause it is mutable
    x.push(2);
    x.push(3);
    println!("{:?}", x); //print vector 

    // another way to creat a vector is using macro
    let ans = vec![1,2,3];
    println!("{:?}", ans);

    let u = evenn(&x);
    println!("{:?}", u);
    let m = vec(&mut x);
    println!("{:?}", m);
    let y = remove(&mut x);
    println!("{:?}", y);
}

fn vec(x: &mut Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    while i < x.len() {
        if x[i] % 2 != 0 {
            x.remove(i);
        } else {
            i += 1;
        }
    }
    x.to_vec()
}

fn evenn(x: &Vec<u32>) -> Vec<u32> {
    let mut new_vec = Vec::new();
    for val in x {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}

fn remove(a: &mut Vec<u32>) -> Vec<u32> {
    a.retain(|&i| i % 2 != 0); //retains return nothing its return type os () to we need to make a copy of vect to return a
    a.to_vec()
}
