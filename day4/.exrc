let s:cpo_save=&cpo
set cpo&vim
inoremap <C-G>S <Plug>(nvim-surround-insert-line)
inoremap <C-G>s <Plug>(nvim-surround-insert)
cnoremap <silent> <Plug>(TelescopeFuzzyCommandSearch) e "lua require('telescope.builtin').command_history { default_text = [=[" . escape(getcmdline(), '"') . "]=] }"
imap <M-C-Right> <Plug>(copilot-accept-line)
imap <M-Right> <Plug>(copilot-accept-word)
imap <M-Bslash> <Plug>(copilot-suggest)
imap <M-[> <Plug>(copilot-previous)
imap <M-]> <Plug>(copilot-next)
imap <Plug>(copilot-suggest) <Cmd>call copilot#Suggest()
imap <Plug>(copilot-previous) <Cmd>call copilot#Previous()
imap <Plug>(copilot-next) <Cmd>call copilot#Next()
imap <Plug>(copilot-dismiss) <Cmd>call copilot#Dismiss()
inoremap <C-U> u
nnoremap  :cnext
nnoremap <NL> :tabmove +1
nnoremap  :tabmove -1
nnoremap  <Cmd>nohlsearch|diffupdate|normal! 
nnoremap  :cprev
nnoremap - :split
nnoremap / :vsplit
nnoremap  o :ClangdSwitchSourceHeader
nnoremap  ch :Telescope git_branches 
vnoremap  tb :Tabularize //l1r1<Left><Left><Left><Left><Left>
nnoremap  tb :Tabularize //l1r1<Left><Left><Left><Left><Left>
nnoremap  cc :CopilotChatToggle
nnoremap  n :tabnew %
nnoremap  q :tabclose
nnoremap  k gT
nnoremap  j gt
nnoremap  s :source
nnoremap  w :w
nnoremap  cd :cd %:h/
xnoremap # y?\V"
omap <silent> % <Plug>(MatchitOperationForward)
xmap <silent> % <Plug>(MatchitVisualForward)
nmap <silent> % <Plug>(MatchitNormalForward)
nnoremap & :&&
xnoremap * y/\V"
nnoremap E :e /home/wojtek/.config/nvim/init.lua
vnoremap J 
nnoremap Q :Ex
xnoremap S <Plug>(nvim-surround-visual)
nnoremap Y y$
omap <silent> [% <Plug>(MatchitOperationMultiBackward)
xmap <silent> [% <Plug>(MatchitVisualMultiBackward)
nmap <silent> [% <Plug>(MatchitNormalMultiBackward)
omap <silent> ]% <Plug>(MatchitOperationMultiForward)
xmap <silent> ]% <Plug>(MatchitVisualMultiForward)
nmap <silent> ]% <Plug>(MatchitNormalMultiForward)
xmap a% <Plug>(MatchitVisualTextObject)
nnoremap cS <Plug>(nvim-surround-change-line)
nnoremap cs <Plug>(nvim-surround-change)
nnoremap ds <Plug>(nvim-surround-delete)
xnoremap gS <Plug>(nvim-surround-visual-line)
xmap gx <Plug>NetrwBrowseXVis
nmap gx <Plug>NetrwBrowseX
omap <silent> g% <Plug>(MatchitOperationBackward)
xmap <silent> g% <Plug>(MatchitVisualBackward)
nmap <silent> g% <Plug>(MatchitNormalBackward)
nmap gcu <Plug>Commentary<Plug>Commentary
nmap gcc <Plug>CommentaryLine
omap gc <Plug>Commentary
nmap gc <Plug>Commentary
xmap gc <Plug>Commentary
tnoremap jk 
nnoremap ySS <Plug>(nvim-surround-normal-cur-line)
nnoremap yS <Plug>(nvim-surround-normal-line)
nnoremap yss <Plug>(nvim-surround-normal-cur)
nnoremap ys <Plug>(nvim-surround-normal)
nnoremap <F11> :DapStepInto
nnoremap <F10> :DapStepOver
nnoremap <F9> :DapToggleBreakpoint
nnoremap <F5> :DapContinue
tnoremap <silent> <Plug>(fzf-normal) 
tnoremap <silent> <Plug>(fzf-insert) i
nnoremap <silent> <Plug>(fzf-normal) <Nop>
nnoremap <silent> <Plug>(fzf-insert) i
xnoremap <silent> <Plug>NetrwBrowseXVis :call netrw#BrowseXVis()
nnoremap <silent> <Plug>NetrwBrowseX :call netrw#BrowseX(netrw#GX(),netrw#CheckIfRemote(netrw#GX()))
xmap <silent> <Plug>(MatchitVisualTextObject) <Plug>(MatchitVisualMultiBackward)o<Plug>(MatchitVisualMultiForward)
onoremap <silent> <Plug>(MatchitOperationMultiForward) :call matchit#MultiMatch("W",  "o")
onoremap <silent> <Plug>(MatchitOperationMultiBackward) :call matchit#MultiMatch("bW", "o")
xnoremap <silent> <Plug>(MatchitVisualMultiForward) :call matchit#MultiMatch("W",  "n")m'gv``
xnoremap <silent> <Plug>(MatchitVisualMultiBackward) :call matchit#MultiMatch("bW", "n")m'gv``
nnoremap <silent> <Plug>(MatchitNormalMultiForward) :call matchit#MultiMatch("W",  "n")
nnoremap <silent> <Plug>(MatchitNormalMultiBackward) :call matchit#MultiMatch("bW", "n")
onoremap <silent> <Plug>(MatchitOperationBackward) :call matchit#Match_wrapper('',0,'o')
onoremap <silent> <Plug>(MatchitOperationForward) :call matchit#Match_wrapper('',1,'o')
xnoremap <silent> <Plug>(MatchitVisualBackward) :call matchit#Match_wrapper('',0,'v')m'gv``
xnoremap <silent> <Plug>(MatchitVisualForward) :call matchit#Match_wrapper('',1,'v'):if col("''") != col("$") | exe ":normal! m'" | endifgv``
nnoremap <silent> <Plug>(MatchitNormalBackward) :call matchit#Match_wrapper('',0,'n')
nnoremap <silent> <Plug>(MatchitNormalForward) :call matchit#Match_wrapper('',1,'n')
nmap <silent> <Plug>CommentaryUndo :echoerr "Change your <Plug>CommentaryUndo map to <Plug>Commentary<Plug>Commentary"
nmap <M-l> <Plug>MoveCharRight
nmap <M-h> <Plug>MoveCharLeft
nmap <M-k> <Plug>MoveLineUp
nmap <M-j> <Plug>MoveLineDown
vmap <M-l> <Plug>MoveBlockRight
vmap <M-h> <Plug>MoveBlockLeft
vmap <M-k> <Plug>MoveBlockUp
vmap <M-j> <Plug>MoveBlockDown
nnoremap <Plug>PlenaryTestFile :lua require('plenary.test_harness').test_file(vim.fn.expand("%:p"))
nnoremap <C-Q> :cprev
nnoremap <C-E> :cnext
nnoremap <C-K> :tabmove -1
nnoremap <C-J> :tabmove +1
nnoremap <Down> -
nnoremap <Up> +
nnoremap <Left> <
nnoremap <Right> >
nnoremap <C-W>- :split
nnoremap <C-W>/ :vsplit
nnoremap <C-L> <Cmd>nohlsearch|diffupdate|normal! 
inoremap S <Plug>(nvim-surround-insert-line)
inoremap s <Plug>(nvim-surround-insert)
inoremap  u
inoremap jk 
let &cpo=s:cpo_save
unlet s:cpo_save
set expandtab
set helplang=en
set ignorecase
set operatorfunc=v:lua.require'nvim-surround'.change_callback
set runtimepath=~/.config/nvim,~/.local/share/nvim/plugged/plenary.nvim,~/.local/share/nvim/plugged/nvim-nio,~/.local/share/nvim/plugged/gruvbox,~/.local/share/nvim/plugged/kanagawa.nvim,~/.local/share/nvim/plugged/vim-code-dark,~/.local/share/nvim/plugged/lualine.nvim,~/.local/share/nvim/plugged/lualine-lsp-progress,~/.local/share/nvim/plugged/vim-devicons,~/.local/share/nvim/plugged/guess-indent.nvim,~/.local/share/nvim/plugged/tabular,~/.local/share/nvim/plugged/nvim-surround,~/.local/share/nvim/plugged/autoclose.nvim,~/.local/share/nvim/plugged/vim-move,~/.local/share/nvim/plugged/nvim-treesitter,~/.local/share/nvim/plugged/nvim-treesitter-textobjects,~/.local/share/nvim/plugged/telescope.nvim,~/.local/share/nvim/plugged/telescope-fzf-native.nvim,~/.local/share/nvim/plugged/vim-commentary,~/.local/share/nvim/plugged/vim-markdown,~/.local/share/nvim/plugged/vim-fugitive,~/.local/share/nvim/plugged/nvim-lspconfig,~/.local/share/nvim/plugged/nvim-cmp,~/.local/share/nvim/plugged/cmp-nvim-lsp,~/.local/share/nvim/plugged/cmp-path,~/.local/share/nvim/plugged/LuaSnip,~/.local/share/nvim/plugged/lsp-zero.nvim,~/.local/share/nvim/plugged/mason-lspconfig.nvim,~/.local/share/nvim/plugged/mason.nvim,~/.local/share/nvim/plugged/copilot.vim,~/.local/share/nvim/plugged/CopilotChat.nvim,~/.local/share/nvim/plugged/nvim-dap,~/.local/share/nvim/plugged/nvim-dap-ui,~/.config/kdedefaults/nvim,/etc/xdg/nvim,~/.local/share/nvim/site,~/.local/share/flatpak/exports/share/nvim/site,/var/lib/flatpak/exports/share/nvim/site,/usr/local/share/nvim/site,/usr/share/nvim/site,/usr/share/nvim/runtime,/usr/share/nvim/runtime/pack/dist/opt/matchit,/usr/lib/nvim,/usr/share/nvim/site/after,/usr/local/share/nvim/site/after,/var/lib/flatpak/exports/share/nvim/site/after,~/.local/share/flatpak/exports/share/nvim/site/after,~/.local/share/nvim/site/after,/etc/xdg/nvim/after,~/.config/kdedefaults/nvim/after,~/.config/nvim/after,~/.local/share/nvim/plugged/tabular/after,~/.local/share/nvim/plugged/vim-markdown/after,~/.local/share/nvim/plugged/cmp-nvim-lsp/after,~/.local/share/nvim/plugged/cmp-path/after,/usr/share/vim/vimfiles
set shiftwidth=4
set softtabstop=4
set splitbelow
set splitright
set statusline=%#Normal#
set tabstop=4
set termguicolors
set window=60
" vim: set ft=vim :
