use std::marker::PhantomData;

struct Locked;
struct Unlocked;

struct PasswordManager<State = Locked> {
    name: String,
    password: String,
    state: PhantomData<State>
}

impl PasswordManager {
    pub fn new() -> Self {
        PasswordManager {
            name: Default::default(),
            password: Default::default(),
            state: PhantomData,
        }
    }
}

impl<State> PasswordManager<State> {
    pub fn say_hi(&self) {
        println!("Hi");
    }

    pub fn print_state(&self) {
        println!("State: {:?}", self.state);
    } 
}

impl PasswordManager<Locked> {
    fn unlock(self) -> PasswordManager<Unlocked> {
        PasswordManager {
            name: self.name,
            password: self.password,
            state: PhantomData
        }
    }
}

impl PasswordManager<Unlocked> {
    fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            name: self.name,
            password: self.password,
            state: PhantomData
        }
    }
}

fn main() {
    let pass_man = PasswordManager::new();
    pass_man.say_hi();
    pass_man.print_state();

    // This will not compile because the function is not implemented for that type! Super type-safe :)
    //pass_man.lock();

    let pass_man: PasswordManager<Unlocked> = pass_man.unlock();
    pass_man.say_hi();
    pass_man.print_state();
    //pass_man.unlock();

    let pass_man = pass_man.lock();
    pass_man.say_hi();
    pass_man.print_state();
}

// The reason you would do this is to make it idiot proof (non-misconfigurable)
// This allows the computer to not have to worry about error checking as much
// Prevents calling state changing functions twice in a row
// Logic errors will be caught by the compiler :)