mod my_mod {
    #[warn(dead_code)]
    fn private_function() -> () {
        print!("called `my_mod::private_function()`");
    }

    pub fn function() -> () {
        print!("called `my_mod::function()`");
    }

    pub mod nested {
        use crate::my_mod::nested;

        pub fn function() -> () {
            print!("called `my_mod::nested::function()`");
        }

        #[warn(dead_code)]
        pub fn private_function() -> () {
            print!("called `my_mod::nested::private_function()`");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod() -> () {
            print!("called `my_mod::nested::public_function_in_my_mod`");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() -> () {
            print!("called `my_mod::nested::public_function_in_nested`");
        }

        pub(super) fn call_public_function_in_my_mod() {
            print!("called `my_mod::call_public_function_in_my_mod()`, that\n>;;;;");
            nested::call_public_function_in_my_mod();
            print!("> ");
            nested::public_function_in_super_mod();
        }

        mod private_nested {
            pub fn function() {
                print!("called `my_mod::private_nested::function()`");
            }
        }
    }

    pub fn call_public_function_in_my_mod() {}
}

fn main() {
    println!("Hello, world!");
}
