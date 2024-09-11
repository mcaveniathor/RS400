use uuid::Uuid;

#[derive(Debug)]
pub struct Library {
    /// Name of this library
    pub name: String,
    pub lib_type: LibraryType,
    /// Uuid of this library
    pub id: Uuid,
    pub description: Option<String>,
    pub authority: Authority,
    pub create_authority: CreateAuthority,
    pub create_object_auditing: CreateObjectAuditing,
}

impl Library {
    pub fn new(lib: String, lib_type: Option<LibraryType>, text: Option<String>, aut: Option<Authority>, crtaut: Option<CreateAuthority>, crtobjaud: Option<CreateObjectAuditing>) -> Library {
        let id = Uuid::new_v4();
        Library {
            name: lib,
            lib_type: lib_type.unwrap_or_default(),
            id,
            description: text,
            authority: aut.unwrap_or_default(),
            create_authority: crtaut.unwrap_or_default(),
            create_object_auditing: crtobjaud.unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub enum Authority {
    LibCreateAuthority,
    Change,
    All,
    Use,
    Exclude,
    AuthList(String)
}
impl Default for Authority {
    fn default() -> Authority {
        Authority::LibCreateAuthority
    }
}
#[derive(Debug)]
pub enum LibraryType {
    Prod,
    Test,
}
impl Default for LibraryType {
    fn default() -> LibraryType {
        LibraryType::Prod
    }
}
#[derive(Debug)]
pub enum CreateAuthority {
    Sysval,
    Change,
    All,
    Use,
    Exclude,
    AuthList(String),
}
impl Default for CreateAuthority {
    fn default() -> CreateAuthority {
        CreateAuthority::Sysval
    }
}
#[derive(Debug)]
pub enum CreateObjectAuditing {
    Sysval,
    _None,
    UsrPrf,
    Change,
    All,
    Use,
    Exclude,
    AuthList(String),
}
impl Default for CreateObjectAuditing {
    fn default() -> CreateObjectAuditing {
        CreateObjectAuditing::Sysval
    }
}



