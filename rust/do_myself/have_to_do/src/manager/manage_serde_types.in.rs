#[derive(Serialize,Deserialize, PartialEq,Eq, Debug)]
struct CrateName {
    name: Option<String>,
    done: bool,
}
#[derive(Serialize,Deserialize,PartialEq,Eq, Debug)]
pub struct Manager {
    list: HashMap<u32, CrateName>,
    achivement: u32,
    obtain_file: String,
    number_of_crate: u32,
}
