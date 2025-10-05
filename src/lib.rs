//! Record values from active stack frames so that they can be printed from a panic handler.
//!
//! # Secondary errors
//!
//! In general this code tries to avoid panicking because that might cause a double panic
//! or might obscure the more interesting original cause of the panic.

use std::{cell::RefCell, thread_local};

thread_local! {
    /// Values observed on currently active stack frames in this thread.
    static VALUES: RefCell<Vec<Frame>> = const { RefCell::new(Vec::new()) };
}

type Value = u64;

/// Holds one observed value.
#[derive(Debug)]
struct Frame {
    name: &'static str,
    value: Value,
    // TODO: Also some metadata: source position, name, etc.
}

/// Pops one frame off the stack when dropped.
pub struct Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        eprintln!("Guard dropped");
        VALUES.with(|values| match values.try_borrow_mut() {
            Ok(mut values) => {
                if values.pop().is_none() {
                    eprintln!("FrameGuard dropped without matching push");
                }
            }
            Err(_) => eprintln!("Failed to borrow values in drop"),
        });
    }
}

/// Remember a value observed on the current stack frame.
pub fn record_value(name: &'static str, value: u64) -> Guard {
    println!("Record value: {}", value);
    VALUES.with(|values| match values.try_borrow_mut() {
        Ok(mut values) => values.push(Frame { name, value }),
        Err(_) => eprintln!("Failed to borrow values in record_value"),
    });
    Guard {}
}

pub fn print_values() {
    VALUES.with(|values| {
        println!("Values:");
        for frame in values.borrow().iter() {
            println!("  {frame:?}");
        }
    });
}

pub fn install_panic_hook() {
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        println!("Panic occurred: {:?}", info);
        print_values();
        prev_hook(info);
    }));
}

#[cfg(test)]
mod tests {
    // use super::*;
}
