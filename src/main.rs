

fn main() {
	let v1 = vec![1,2,3,4];
	let v2 = vec![2,2,3,4]; // First common element is at index 1 in v1 and 1 in v2
	
	let result = first_common_index(&v1, &v2);
	println!("{}", result);
	println!("{:?}", &v1);
	println!("{:?}", &v2);
}

pub fn first_common_index(v1: &Vec<i32>, v2: &Vec<i32>)-> usize{
    if v1.len() == 0{
        return 0;
    }
    else if v2.len() == 0{
        return v1.len();
    }
    let shorten_len = if v1.len() < v2.len(){
        v1.len()
    }else {
        v2.len()
    };
    for i in 0..shorten_len{
        if v1[i]== v2[i]{
            return i;
        }
    }
    return v1.len();
}
