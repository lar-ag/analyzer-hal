/// Monitor gear pump normally used for solution sampling. 
/// 
/// 


pub trait Pump {
    
    type Error;
    type Config;

    fn run( &mut self)-> Result<(), Self::Error>;
    fn stop( &mut self)->Result<(), Self::Error>;
}



