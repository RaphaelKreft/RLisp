// Ititial test env
use std::collections::HashMap;

struct Env {   
    env: HashMap<String, Sexpr>,
}

impl Env {
    fn initGlobal() -> Env {
        let genv = Env{env: HashMap::new()}:
        genv.set("car", );
        genv.set("cdr", );
        genv.set("cons", );
        genv.set("+", );
        genv.set("-", );
        return genv;
    }

    fn set(&self, symbol: String, expr: Sexpr) {

    }

    fn get(&self) {

    }

    fn find(&self) {

    }
}
