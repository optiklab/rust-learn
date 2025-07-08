pub mod gui {

    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>> // any type inside a Box that 
        // implements the Draw trait. This works differently from defining 
        // a struct that uses a generic type parameter with trait bounds. 
        // A generic type parameter can be substituted with only one 
        // concrete type at a time, whereas trait objects allow for multiple 
        // concrete types to fill in for the trait object at runtime.
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // code to actually draw a button
        }
    }

    //////////////////////////////////////////////////////////////////////////////
    // An alternate implementation of the Screen struct and its run method using 
    // generics and trait bounds. This restricts us to a Screen instance that has 
    // a list of components all of type Button or all of type TextField. If you’ll 
    // only ever have homogeneous collections, using generics and trait bounds is 
    // preferable because the definitions will be monomorphized at compile time 
    // to use the concrete types.
    pub struct ScreenGeneric<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> ScreenGeneric<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    ///////////////////////////////////////////////////////////////////////////////

}