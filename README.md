timeit
======

Minimalist timing application, useful for profiling your init scripts.

## Usage

```
$ timeit 'demo' # starts timer
$ somedemo
$ timeit 'demo' # ends timer
demo 1896649916
# time is printed in nanoseconds
```

## Install

You can grab `timeit` from crates.io via Cargo, the package is called `timeit-tool`.

```
$ cargo install timeit-tool
$ which timeit # /home/mt/.cargo/bin/timeit on my machine
```

## License

timeit is MIT licensed.

&copy; 2016, [Marinin Tim](http://marinin.xyz)
