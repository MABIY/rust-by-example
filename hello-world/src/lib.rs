
/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}
impl Person {

    /// Returns a person with the name given them
    ///
    /// # Arguments
    /// * `name` - A String slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments /// // if you pass --test  t0 `rustdoc`, it will even test it for you ! /// use doc::Person
    /// use doc::Person;
    /// let person = Person::new("name");
    ///
    /// ````
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_owned(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name](Person::name)" to the `Person` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!",self.name);
    }

}

#[allow(dead_code)]
fn main() {
    let john = Person::new("John");
    john.hello();
}
