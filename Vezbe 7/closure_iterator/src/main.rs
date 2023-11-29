mod closure;
mod closure_and_ref;
mod closure_fn;
mod iterator;

fn main() {
    println!("Closure");
    closure::closure_example();

    println!("Closure i reference");
    closure_and_ref::closure();

    println!("Closure Fn");
    closure_fn::fn_mut();
    closure_fn::fn_mut_counter();



    println!("-----------------------------------");
    println!("Iteratori");

    iterator::iterator();

    iterator::adapters();

    iterator::method_adapters();


}
