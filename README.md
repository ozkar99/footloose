# footloose

footloose is a (windows only) small utiliy that transforms midi CC messages into windows inputs (keystrokes, mouse clicks, mouse movements, etc...),
it is configured by editing its source code, its pre-configured with the following:

```
53 => ctrl+c
52 => ctrl+v
64 => ctrl+z
```

footloose does not care for program changes or the values of the control changes messages, although this could easily be extended for that use-case.
