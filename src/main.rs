fn main(){
	println!("sdhfs12492839483924dfujd");
	let a:String = "Hello".to_string();
	let b=String::from("hello");
	let c = "World".to_owned();
	let d = c.clone();
	println!("{0},{1},{2},{3},{0}",a,b,c,d);
	
	let x = 1_i32;
	let add_x = |a| {x+a};
	let result = add_x(5);
	println!("{}",result);
	
	let mut sum = 0;
	
	for i in (0..1000){
		if i%3 ==0 || i%5 ==0{
			sum += i;
		}
	}
	
	sum = 0;
	
	let mut a = 1;
	let mut b = 1;
	let mut c = 1;
	println!("{},{},",a,b);
	while c< 4000000{
		c = a+b;
		//println!("{}",c);
		b = a;
		a = c;

		if c%2 ==0{
			sum += c;
		}
	}

	
	println!("{}",sum);
}