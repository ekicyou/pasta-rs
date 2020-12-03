use async_trait::*;

#[async_trait]
pub trait Scriptor {
    fn actor(&mut self, t: &str);
    fn action(&mut self, t: &str);
    fn serif(&mut self, t: &str);
    async fn start(&mut self);
    fn tags(&self) -> &[String];
}
