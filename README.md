# Segfault

This segfaults on my Ubuntu 18.04

```
export LIBGL_ALWAYS_SOFTWARE=1 LIBGL_DEBUG=verbose
cargo test
```

Output on my machine:

```
$ cargo test
   Compiling amethyst_gl_segfault v0.1.0 (file:///home/azriel/work/github/azriel91/amethyst_gl_segfault)
    Finished dev [unoptimized + debuginfo] target(s) in 7.29s
     Running target/debug/deps/amethyst_gl_segfault-8e9c5ca4ba7c6deb

running 1 test
libGL: OpenDriver: trying /usr/lib/x86_64-linux-gnu/dri/tls/swrast_dri.so
libGL: OpenDriver: trying /usr/lib/x86_64-linux-gnu/dri/swrast_dri.so
libGL: Can't open configuration file /home/azriel/.drirc: No such file or directory.
libGL: Can't open configuration file /home/azriel/.drirc: No such file or directory.
[amethyst::app][INFO] Initializing Amethyst...
[amethyst::app][INFO] Version: 0.7.0
[amethyst::app][INFO] Platform: x86_64-unknown-linux-gnu
[amethyst::app][INFO] Amethyst git commit: 00fa904612be5609121981a9663a7aea097e3a55
[amethyst::app][INFO] Rustc version: 1.28.0-nightly Nightly
[amethyst::app][INFO] Rustc git commit: b68432d560c7c6f1e738b27e49d271a2a778f898
[amethyst_assets::storage][DEBUG] "renderer::Texture": Asset "<Data>" (handle id: Handle { id: 0, marker: PhantomData }) has been loaded successfully
[amethyst_assets::storage][DEBUG] "renderer::Texture": Asset "<Data>" (handle id: Handle { id: 1, marker: PhantomData }) has been loaded successfully
[amethyst_assets::storage][DEBUG] "renderer::Texture": Asset "<Data>" (handle id: Handle { id: 2, marker: PhantomData }) has been loaded successfully
[amethyst_assets::storage][DEBUG] "renderer::Texture": Asset "<Data>" (handle id: Handle { id: 3, marker: PhantomData }) has been loaded successfully
[amethyst_assets::storage][DEBUG] "renderer::Texture": Asset "<Data>" (handle id: Handle { id: 4, marker: PhantomData }) has been loaded successfully
[amethyst_assets::storage][DEBUG] "renderer::Texture": Asset "<Data>" (handle id: Handle { id: 5, marker: PhantomData }) has been loaded successfully
[amethyst_assets::storage][DEBUG] "renderer::Texture": Asset "<Data>" (handle id: Handle { id: 6, marker: PhantomData }) has been loaded successfully
[amethyst::app][INFO] Engine is shutting down
test tests::segfault ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

error: process didn't exit successfully: `/home/azriel/work/github/azriel91/amethyst_gl_segfault/target/debug/deps/amethyst_gl_segfault-8e9c5ca4ba7c6deb` (signal: 11, SIGSEGV: invalid memory reference)
```
