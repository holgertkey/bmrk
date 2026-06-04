# bm.sh — shell wrapper for bmrk (bash / zsh / fish)
#
# The TUI renders to stderr (visible in terminal); stdout carries the path.
# This wrapper captures stdout, and if it is a directory, cd's to it.
# All other output (help, version, bookmark list, confirmations) is printed normally.
#
# INSTALLATION
# ────────────
# bash / zsh — add ONE of these lines to ~/.bashrc or ~/.zshrc:
#
#   source /path/to/bm.sh
#
#   — or inline (no extra file needed):
#
#   bm() { local r; r=$(bmrk "$@"); [ -d "$r" ] && cd "$r" || { [ -n "$r" ] && echo "$r"; }; }
#
# fish — create ~/.config/fish/functions/bm.fish with:
#
#   function bm
#       set r (bmrk $argv)
#       if test -d "$r"; cd $r
#       else if test -n "$r"; echo $r
#       end
#   end
#
# After editing your shell config, reload it:
#   source ~/.bashrc    # bash
#   source ~/.zshrc     # zsh
#   # fish reloads automatically

bm() {
    local result exit_code
    result=$(bmrk "$@")
    exit_code=$?
    if [ -n "$result" ] && [ -d "$result" ]; then
        cd "$result" || return 1
    elif [ -n "$result" ]; then
        echo "$result"
    fi
    return $exit_code
}
