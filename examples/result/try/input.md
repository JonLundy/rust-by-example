Chaining results using match can get pretty untidy; luckily, the `try!` macro
can be used to make things pretty again. The `try!` macro expands to a match
expression, where the `Err(err)` branch expands to an early `return err`, and
the `Ok(ok)` branch expands to an `ok` expression.

{try.play}

Be sure to check the [documentation](http://doc.rust-lang.org/std/result/type.Result.html),
as there are many methods to map/compose `Result`.
