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
#   bm() { local r x="$PWD"; if [ "$1" = "-" ]; then [ -n "$BMRK_PREV_DIR" ] && { cd "$BMRK_PREV_DIR" && BMRK_PREV_DIR="$x"; } || { echo "bm: no previous directory" >&2; return 1; }; return; fi; r=$(bmrk "$@"); if [ -n "$r" ] && [ -d "$r" ]; then cd "$r" && BMRK_PREV_DIR="$x"; elif [ -n "$r" ]; then echo "$r"; fi; }
#
# fish — create ~/.config/fish/functions/bm.fish with:
#
#   function bm
#       if test "$argv[1]" = "-"
#           if test -n "$BMRK_PREV_DIR" -a -d "$BMRK_PREV_DIR"
#               set prev $BMRK_PREV_DIR
#               set -gx BMRK_PREV_DIR $PWD
#               cd $prev
#           else
#               echo "bm: no previous directory" >&2; return 1
#           end
#           return
#       end
#       set r (bmrk $argv)
#       if test -d "$r"
#           set -gx BMRK_PREV_DIR $PWD; cd $r
#       else if test -n "$r"
#           echo $r
#       end
#   end
#
# After editing your shell config, reload it:
#   source ~/.bashrc    # bash
#   source ~/.zshrc     # zsh
#   # fish reloads automatically

bm() {
    local result prev_dir="$PWD"

    # Return to previous directory
    if [ "$1" = "-" ]; then
        if [ -n "$BMRK_PREV_DIR" ] && [ -d "$BMRK_PREV_DIR" ]; then
            cd "$BMRK_PREV_DIR" || return 1
            BMRK_PREV_DIR="$prev_dir"
        else
            echo "bm: no previous directory" >&2
            return 1
        fi
        return
    fi

    result=$(bmrk "$@")
    local exit_code=$?

    if [ -n "$result" ] && [ -d "$result" ]; then
        cd "$result" || return 1
        BMRK_PREV_DIR="$prev_dir"
    elif [ -n "$result" ]; then
        echo "$result"
    fi

    return $exit_code
}
