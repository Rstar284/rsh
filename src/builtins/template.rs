use crate::eval::CmdErr;

pub trait Cmd {
    fn name() -> &'static str;
    fn about() -> &'static str;
    fn examples() -> [&'static str; 3];
    fn run(args: Vec<String>) -> Result<(), CmdErr>;
    fn help(&self) -> String {
        format!("{}\nAbout: \n{}\nExamples: \n1. {}\n2. {}\n3. {}", Self::name(), Self::about(), Self::examples()[0], Self::examples()[1], Self::examples()[2])
    }
}