# A very simple BF Interpreter using Rust

<p>Run using: <code>cargo run -- path/to/file</code> <br>
to run in release mode: <code>cargo run --release</code><br>
to run mainv2: <code>cargo run --release --bin mainv2</code></p>

<p>To check performance:<br>
for main.rs: `multitime -n 10 -s0 target/release/bf_interpreter path/to/TestFile`<br>
for mainv2.rs: `multitime -n 10 -s0 target/release/mainv2 /path/to/TestFile`</p>
