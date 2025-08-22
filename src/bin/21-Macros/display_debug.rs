/* this is just example how to understand the original code doesnt look like this can you dont have to write it you can use a debug macro */

/* display macro doesnt exist so you can use debug everything that doesnt have display */


/* Display */
// this is print using {}
// "human-friendly printing" ({})
// Used when you want pretty, user-facing output.

struct User {
    username: String,
}

impl Display for User { // this is a debug trait -- this implements on the code but doesnt expand the code -- like the debug macro
        fn fmt(&self,f: &mut Formatter<'_> ) -> std::fmt::Result {
            write!(f,"{}", self.username)
        }
}


/* Debug */
// this is print using {:?}
// "developer-friendly printing" ({:?} or {:#?})
// Used for debugging, when you just want to see the internal state.

struct User {
    username: String,
}

impl Debug for User {
        fn fmt(&self,f: &mut Formatter<'_> ) -> std::fmt::Result {
            write!(f,"{}", self.username)
        }
}

/* we can use both {} {:?} to print anything using the and debug macro */