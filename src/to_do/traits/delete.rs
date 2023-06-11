pub trait Delete{
    fn create(&self, title: &str){
        println!("{} to do is deleted", title);
    }
}