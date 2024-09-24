pub struct Profile {
    // Note, this is a slight divergence from the spec,
    // spec specifies id should be int
    // but since all the other ids are in uint64, I decided to unify everything as uint64
    pub id: u64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

impl From<crate::repository::model::Profile> for Profile {
    fn from(value: crate::repository::model::Profile) -> Self {
        Profile {
            id: value.id,
            email: value.email,
            firstname: value.firstname,
            lastname: value.lastname,
        }
    }
}
