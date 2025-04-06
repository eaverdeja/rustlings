trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// Short-hand with impl
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

// Using generic types
fn some_func_generic<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

// Using the where trait bound syntax
fn some_func_where<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait,
{
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }

    fn test_some_func_generic() {
        assert!(some_func_generic(SomeStruct));
        assert!(some_func_generic(OtherStruct));
    }

    fn test_some_func_where() {
        assert!(some_func_where(SomeStruct));
        assert!(some_func_where(OtherStruct));
    }
}
