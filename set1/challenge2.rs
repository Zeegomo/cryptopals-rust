use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2{
        let a = &args[1];
        let b = &args[2];

        hex_xor(a,b);
    }

}
fn hex_xor(a: &str, b: &str){
    if a.len() != b.len() {
        println!("Lenght error");
    }else{

        //using for to handle arbitrary large numbers without storing the entire value
        for i in 0..a.len(){
            let mut d = 0;
            let mut c = 0;

            match u64::from_str_radix(&a[i..i+1],16){
                Ok(n) => d = n,
                Err(_) => println!(),
            }
            match u64::from_str_radix(&b[i..i+1],16){
                Ok(n) => c = n,
                Err(_) => println!(),
            }

            print!("{:x}",d ^ c);
        }
    }
}