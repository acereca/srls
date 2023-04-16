**⚠ This language server is still in it infancy. It only partially supports the SKILL language and can break quite easily during usage. ⚠**

# SKILL Rust Language Server - SRLS

a language server for [Cadence SKILL](https://en.wikipedia.org/wiki/Cadence_SKILL), 

## Features

### Variable completion

![](https://git.acereca.net/acereca/srls/raw/branch/master/assets/variable_completion.GIF)

variables assigned to using the infix `=` operator can be completed anywhere

#### Custom docstrings

```lisp
;;; this is a custom docstring supported for variable definitions
variable = "some content"
```

this allows for the docstring to show during completion

