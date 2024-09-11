use uuid::Uuid;


pub trait Object {
    fn name(&self) -> String;
    fn lib(&self) -> String;
    fn id(&self) -> Uuid;
    fn obj_type(&self) -> ObjectType;
}


#[derive(Debug)]
pub enum ObjectType {
    Library,
    LibraryList,
    Program,
    Command,
    Menu,
    File,
    UserProfile,
    SubsystemDescription,
    JobDescription,
    JobQueue,
    MessageQueue,
    OutQueue,
    Other(String),
}


