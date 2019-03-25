#[derive(Debug)]
pub enum Error {
    WrongRule,
    IllegalToken,
    MissingOperator,
}

fn stuff() {
    #cfg(mac)
    {
      // here
    } 
    cfg(not(mac))
    {
      // there
    }
}