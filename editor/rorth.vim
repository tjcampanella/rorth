" Vim syntax file
" Language: Rorth

" Usage Instructions
" Put this file in .vim/syntax/rorth.vim
" and add in your .vimrc file the next line:
" autocmd BufRead,BufNewFile *.rorth set filetype=rorth

if exists("b:current_syntax")
  finish
endif

set iskeyword=a-z,A-Z,-,*,_,!,@
syntax keyword rorthTodos TODO XXX FIXME NOTE

" Language keywords
syntax keyword rorthKeywords if else while do include memory fn const end assert

" Comments
syntax region rorthCommentLine start="//" end="$"   contains=rorthTodos

" String literals
syntax region rorthString start=/\v"/ skip=/\v\\./ end=/\v"/ contains=rorthEscapes

" Char literals
syntax region rorthChar start=/\v'/ skip=/\v\\./ end=/\v'/ contains=rorthEscapes

" Escape literals \n, \r, ....
syntax match rorthEscapes display contained "\\[nr\"']"

" Number literals
syntax region rorthNumber start=/\s\d/ skip=/\d/ end=/\s/

" Type names the compiler recognizes
syntax keyword rorthTypeNames addr int ptr bool
" Set highlights
highlight default link rorthTodos Todo
highlight default link rorthKeywords Keyword
highlight default link rorthCommentLine Comment
highlight default link rorthString String
highlight default link rorthNumber Number
highlight default link rorthTypeNames Type
highlight default link rorthChar Character
highlight default link rorthEscapes SpecialChar

let b:current_syntax = "rorth"

