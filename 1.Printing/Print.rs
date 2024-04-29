
fn main(){
	println!("Hello world !");

	println!("I am learning {language} in order to do {project} with my friend {name}. This is a Debug test {name:?}",
			language="Rust",
			project="Taskmaster",
			name="Thibault");
	eprintln!("This is an error message which is printed on the standard error.");

 	//This will output "    1" : 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5; 
    println!("{number:>width$}");
}