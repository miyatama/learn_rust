mod logics;
use logics::function_lifetime::run_func_lifetime;
use logics::lifetime_bounds::run_lifetime_bound;
use logics::struct_lifetime::run_struct_lifetime;

pub fn run() {
    run_lifetime_bound();
    run_struct_lifetime();
    run_func_lifetime();
}
