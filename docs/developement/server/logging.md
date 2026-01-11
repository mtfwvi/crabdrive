# Logging

### Log Levels

- Trace
- Debug
- Info
- Warn
- Error

When building in Debug-Mode, all log messages will be printed to the console. In Release however, log messages below `warning` are ignored.

---

Use the `tracing` crate and the corresponding macros:

```rust
use tracing::{error, info, warn, debug, trace};

fn my_method() {
    debug!("Debug Message!");
    trace!("Where is the bug???");
    info!("This message is printed @ INFO level!");
    let user_id = 12345;
    warn!(user_id, "User has no quota left!");
    error!(error = "DatabaseError", "An error occured!");
    info!("My_Method finished!");
}
```

The example prints out:

```log
2025-12-30T19:30:02.000925Z DEBUG ***: Debug Message!
2025-12-30T19:30:02.001481Z TRACE ***: Where is the bug???
2025-12-30T19:30:02.001820Z  INFO ***: This message is printed @ INFO level!
2025-12-30T19:30:02.002257Z  WARN ***: User has no quota left! user_id=12345
2025-12-30T19:30:02.002548Z ERROR ***: An error occured! error="DatabaseError"
2025-12-30T19:30:02.002853Z  INFO ***: My_Method finished!
```

For structural logging you can use spans:

```rust
use tracing::{error, info, instrument, warn, debug};

#[instrument] // Auto generates a span for the function
fn my_method2(user_id: i32) {
    info!("This message is printed @ INFO level!");
    if user_id == 0 {
        warn!(user_id, "User has no quota left!");
    }
    error!(error = "DatabaseError", "An error occured!");
    info!("My_Method finished!");
}
```

Output:

```log
2025-12-30T19:30:02.003287Z  INFO my_method2{user_id=0}: ***:: This message is printed @ INFO level!
2025-12-30T19:30:02.003504Z  WARN my_method2{user_id=0}: ***:: User has no quota left! user_id=0
2025-12-30T19:30:02.003632Z ERROR my_method2{user_id=0}: ***:: An error occured! error="DatabaseError"
2025-12-30T19:30:02.003771Z  INFO my_method2{user_id=0}: ***:: My_Method finished!
```

> [!NOTE]
> Every Request has its own unique Request ID and span. When a request handler calls another function, that function inherits the span/context.

You can also manually create a (nested) span:

```rust
use tracing::{error, info, instrument, warn, debug, debug_span};

fn my_method3(user_id: i32) {
    let span = debug_span!("my_method3", user_id = user_id);
    // Important: When entering the span, assign to span variable to
    //            prevent instant drop! 
    let _enter = span.enter();
    info!("This message is printed @ INFO level!");
    if user_id == 0 {
        warn!(user_id, "User has no quota left!");
    }
    error!(error = "DatabaseError", "An error occured!");
    info!("My_Method finished!");

    let child_span = debug_span!("my_method4", user_id = user_id);
    let _enter2 = child_span.enter();

    my_method4(user_id);
}

fn my_method4(user_id: i32) {
    info!("This message is printed @ INFO level!");
    if user_id == 0 {
        warn!(user_id, "User has no quota left!");
    }
    error!(error = "DatabaseError", "An error occured!");
    info!("My_Method finished!");
}
```

Output:

````log
2025-12-30T19:42:38.891191Z  INFO my_method3{user_id=0}: ***: This message is printed @ INFO level!
2025-12-30T19:42:38.891271Z  WARN my_method3{user_id=0}: ***: User has no quota left! user_id=0
2025-12-30T19:42:38.891344Z ERROR my_method3{user_id=0}: ***: An error occured! error="DatabaseError"
2025-12-30T19:42:38.891414Z  INFO my_method3{user_id=0}: ***: My_Method finished!
2025-12-30T19:42:38.891490Z  INFO my_method3{user_id=0}:my_method4{user_id=0}: ***: This message is printed @ INFO level!
2025-12-30T19:42:38.891563Z  WARN my_method3{user_id=0}:my_method4{user_id=0}: ***: User has no quota left! user_id=0
2025-12-30T19:42:38.891639Z ERROR my_method3{user_id=0}:my_method4{user_id=0}: ***: An error occured! error="DatabaseError"
2025-12-30T19:42:38.891714Z  INFO my_method3{user_id=0}:my_method4{user_id=0}: ***: My_Method finished!
```
