use crate::configuration::Settings;

pub trait Command {
    fn execute(settings: Settings, arguments: Option<Vec<String>>);
}

