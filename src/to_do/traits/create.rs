pub trait Create{
    fn create(&self, title: &str){
        println!("{} to do is created", title);
    }
}