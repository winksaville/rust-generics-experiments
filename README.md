# Generics Experiments

Experiment with rust generics. The makefile will build
`generics` in debug mode by default and in release mode
if build=release is passed on the command line.


# clean all
```
make cleanall
```

# clean current build, default is debug
```
make clean
```
or clean release:
```
make clean build=release
```

# Build default, which is debug
```
make
```
or build release with:
```
make build=release
```

# Run
```
wink@3900x:~/prgs/rust/tutorial/generics (master)
$ ./generics 
The largest number is 100
The largest char is y
print pnt_int with {} { x=5, y=10 }
print pnt_int with {:?} Point { x: 5, y: 10 }
print pnt_int with {:#?} Point {
    x: 5,
    y: 10,
}
print pnt_int with {} { x=5, y=10 }
print pnt_int with {:?} Point { x: 5.0, y: 10.0 }
print pnt_int with {:#?} Point {
    x: 5.0,
    y: 10.0,
}
print pnt_int with {} { x=5, y=10 }
print pnt_int with {:?} Point { x: 5.0, y: 10 }
print pnt_int with {:#?} Point {
    x: 5.0,
    y: 10,
}
```
