pub trait Entity {
    fn name(&self) -> String;
    fn descr(&self) -> String;

    fn health(&self) -> i16;
    fn stamina(&self) -> i16;
    fn magic(&self) -> i16;

    fn damage(&mut self);
    fn heal(&mut self);
}


pub trait Ranged {
    fn ranged_attack(&self, target: Box<dyn Entity>) {

    }
}