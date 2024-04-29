use std::fmt; // importing the fmt module


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

	println!("{:#?}", number); // supposed to be a pretty print but I dont see the difference ?


	// In order to be able to print a structure, we need to precise if it is
	// either with fmt::Display or fmt::Debug with the attribute derive
	// struct NeverPrintable(i32); 

	// #[derive(Debug)]
	// struct DebugPrintable(i32);

	// #[derive(Debug)]
	// struct DebugDouble(DebugPrintable);

	// // #[derive(Display)]
	// // struct DisplayPrintable(i32);

	// println!("{:?} months in the year", DebugDouble(DebugPrintable(4)));
}
