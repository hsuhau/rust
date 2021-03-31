fn main() {
   let company:&str = "TutorialsPoint";
   let location:&str = "Hyderabad";
   println!("company is : {}, location is : {}", company, location);
   
   let company_1:&'static str = "TutorialsPoint";
   let location_1:&'static str = "Hyderabad";
   println!("company is : {}, location is : {}",company_1,location_1);

   let empty_string = String::new();
   println!("length is : {}", empty_string.len());

   let content_string = String::from("TutorialsPoint");
   println!("length is : {}", content_string.len());
}