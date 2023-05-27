**⚠ This language server is still in its infancy. It only partially supports the SKILL language and can break quite easily during usage. ⚠**

# SKILL Rust Language Server - SRLS

a language server for [Cadence SKILL](https://en.wikipedia.org/wiki/Cadence_SKILL), 

## Features

### Variable completion

![](https://github.com/acereca/srls/raw/master/assets/variable_completion.GIF)

variables assigned to using the infix `=` operator can be completed anywhere

- both global and local
- local variables are only completed within their scope

#### Custom docstrings

```lisp
;;; this is a custom docstring supported for variable definitions
variable = "some content"
```

this allows for the docstring to show during completion

## Installation

### neovim (lua)

using [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig)

install `srls` into your path or give the `cmd` table entry the absolute path:

```lua
require('lspconfig.configs').srls = {
    default_config = {
        cmd = {"srls"},
        filetypes = {"skill"},
        root_dir = require('lspconfig.util').root_pattern(".git")
    }
}
require('lspconfig').srls.setup({})
```

#### astronvim

```lua
return {
  lsp = {
    servers = {
      "skill_ls"
    },
    config = {
      skill_ls = function()
        return {
          cmd = { "srls" },
          filetypes = { "skill" },
          root_dir = require('lspconfig.util').root_pattern(".git"),
        }
      end,
    }
  }
}
```

