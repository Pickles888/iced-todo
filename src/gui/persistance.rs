pub trait Persistance {
    fn save() -> Result<(), SaveError> {

    }

    fn load() -> Result<PathBuff, LoadError>
}

#[derive(Debug)]
pub enum SaveError {}

#[derive(Debug)]
pub enum LoadError {}
