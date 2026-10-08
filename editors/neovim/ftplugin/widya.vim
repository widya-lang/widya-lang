" Vim / Neovim Filetype Plugin untuk Widya-Lang
" Lisensi: MIT

if exists("b:did_ftplugin")
  finish
endif
let b:did_ftplugin = 1

setlocal tabstop=4
setlocal softtabstop=4
setlocal shiftwidth=4
setlocal expandtab
setlocal autoindent
setlocal smartindent
setlocal commentstring=//\ %s
setlocal formatoptions-=t formatoptions+=croql

let b:undo_ftplugin = "setlocal tabstop< softtabstop< shiftwidth< expandtab< autoindent< smartindent< commentstring< formatoptions<"
