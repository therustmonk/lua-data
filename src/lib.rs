extern crate lua;

pub use lua::{Integer, Number, ToLua, FromLua};

pub struct Table {
//    indices: Vec<Data>,
//    fileds
}
// TODO Override index operator


pub enum Data {
    String(String),
    Integer(Integer),
    Number(Number),
    Table(Table),
    Boolean(bool),
    Function,
    UserData,
    LightUserData,
    Nil,
}

impl ToLua for Data {
    fn to_lua(&self, state: &mut State) {
        match *self {
            Data::String(ref value) => state.push_string(value),
            Data::Integer(value) => state.push_integer(value),
            Data::Number(value) => state.push_number(value),
            Data::Boolean(value) => state.push_bool(value),
            _ => (),
            Data::Nil => state.push_nil(),
        }
    }
}

impl FromLua for Data {
    fn from_lua(state: &mut State, index: Index) -> Option<Data> {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
