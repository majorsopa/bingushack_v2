pub mod prelude {

    /// we need to access:
    /// - the names of every class we want to edit (FQN)
    /// - name of function we want to inject into
    /// - location of where to inject
    todo!();
}



/// For use with injection
enum At {
    HEAD,
    RETURN, // any return
    TAIL, // last return only   
    // INVOKE(Invoke)
}



// enum Shift {
//     BEFORE,
//     AFTER
// }

// pub struct Invoke {
//     target: String,
//     ordinal: i8,
//     shift: Shift

// }


// What we want:
// #[Inject(At::HEAD)]
// #[Inject(At::RETURN)]
// #[Inject(At::TAIL)]



trait Mixin {


    fn get_java_class_name() -> &'static str {
        todo!();
    }



    fn get_modified_java_functions() -> String /* some sort of type holding java native name and our proc-macro'd native function */ {
        todo!();
    }


    fn inject(/* java shit */) {
        todo!();
    }


}

#[derive(Mixin)]
struct PlayerMixin {

}


impl PlayerMixin {


    #[Inject("/tick()V", At::HEAD)]
    fn custom_tick( /* java shit */) {

    }


}
