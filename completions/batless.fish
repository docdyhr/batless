complete -c batless -l language -d 'Language for syntax highlighting (auto-detect if not specified)' -r
complete -c batless -l max-lines -d 'Limit lines shown' -r
complete -c batless -l max-bytes -d 'Limit bytes shown' -r
complete -c batless -l mode -d 'Output mode' -r -f -a "plain\t''
json\t''
index\t'Machine-readable symbol index with line ranges'"
complete -c batless -l color -d 'Color output control' -r -f -a "auto\t''
always\t''
never\t''"
complete -c batless -l generate-completions -d 'Generate shell completions for the specified shell' -r -f -a "bash\t''
zsh\t''
fish\t''
power-shell\t''"
complete -c batless -l config -d 'Configuration file path (defaults to auto-discovery)' -r
complete -c batless -l strip-ansi -d 'Strip ANSI escape codes from output'
complete -c batless -l list-languages -d 'List all supported languages'
complete -c batless -l debug -d 'Enable debug mode with detailed processing information'
complete -c batless -l plain -d 'PAGER compatibility: equivalent to --mode plain (for cat replacement)'
complete -c batless -s u -l unbuffered -d 'PAGER compatibility: ignored for compatibility with other pagers'
complete -c batless -s n -l number -d 'CAT compatibility: show line numbers (like cat -n)'
complete -c batless -s b -l number-nonblank -d 'CAT compatibility: number non-blank output lines (like cat -b)'
complete -c batless -l no-title -d 'PAGER compatibility: ignored for compatibility with less (no title bar)'
complete -c batless -l version-json -d 'Output version information as machine-readable JSON'
complete -c batless -l json-pretty -d 'Pretty-print JSON output (when --mode=json)'
complete -c batless -l with-line-numbers -d 'Include 1-based line numbers in JSON output lines array (e.g. {"n":1,"text":"..."})'
complete -c batless -l strip-comments -d 'Strip comment-only lines from output'
complete -c batless -l strip-blank-lines -d 'Strip blank lines from output'
complete -c batless -s h -l help -d 'Print help (see more with \'--help\')'
complete -c batless -s V -l version -d 'Print version'
