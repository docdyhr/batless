complete -c batless -l language -d 'Language for syntax highlighting (auto-detect if not specified)' -r
complete -c batless -l max-lines -d 'Limit lines shown' -r
complete -c batless -l max-bytes -d 'Limit bytes shown' -r
complete -c batless -l mode -d 'Output mode' -r -f -a "plain\t''
highlight\t''
json\t''
summary\t''"
complete -c batless -l color -d 'Color output control' -r -f -a "auto\t''
always\t''
never\t''"
complete -c batless -l theme -d 'Theme for syntax highlighting' -r
complete -c batless -l generate-completions -d 'Generate shell completions for the specified shell' -r -f -a "bash\t''
zsh\t''
fish\t''
power-shell\t''"
complete -c batless -l profile -d 'Use predefined AI tool profile (overrides other settings)' -r -f -a "claude\t'Optimized for Claude\'s context window (4K lines, summary mode)'
copilot\t'Focused on code suggestions for GitHub Copilot (2K lines, tokens included)'
chatgpt\t'OpenAI ChatGPT optimizations (3K lines, JSON output)'
assistant\t'General AI assistant profile (5K lines, balanced output)'"
complete -c batless -l strip-ansi -d 'Strip ANSI escape codes from output'
complete -c batless -l list-languages -d 'List all supported languages'
complete -c batless -l list-themes -d 'List all available themes'
complete -c batless -l include-tokens -d 'Include tokens in JSON output (AI-friendly)'
complete -c batless -l summary -d 'Summary mode: show only important code structures'
complete -c batless -s h -l help -d 'Print help (see more with \'--help\')'
complete -c batless -s V -l version -d 'Print version'
