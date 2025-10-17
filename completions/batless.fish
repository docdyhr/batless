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
complete -c batless -l summary-level -d 'Summary level: control detail level of summary output' -r -f -a "none\t'No summary, show full file'
minimal\t'Minimal summary with only critical structures'
standard\t'Standard summary with most important code'
detailed\t'Detailed summary with comprehensive information'"
complete -c batless -l ai-model -d 'AI model for token counting' -r -f -a "gpt4\t'OpenAI GPT-4 family'
gpt4-turbo\t'OpenAI GPT-4 Turbo with enhanced capabilities'
gpt35\t'OpenAI GPT-3.5 family'
claude\t'Anthropic Claude family'
claude35-sonnet\t'Anthropic Claude-3.5 Sonnet with enhanced capabilities'
generic\t'Generic model estimation'"
complete -c batless -l prompt-tokens -d 'Estimate prompt token overhead when fitting context' -r
complete -c batless -l get-schema -d 'Get JSON schema for specified output format' -r
complete -c batless -l generate-completions -d 'Generate shell completions for the specified shell' -r -f -a "bash\t''
zsh\t''
fish\t''
power-shell\t''"
complete -c batless -l profile -d 'Use predefined AI tool profile (overrides other settings)' -r -f -a "claude\t'Optimized for Claude\'s context window (4K lines, summary mode)'
copilot\t'Focused on code suggestions for GitHub Copilot (2K lines, tokens included)'
chatgpt\t'OpenAI ChatGPT optimizations (3K lines, JSON output)'
assistant\t'General AI assistant profile (5K lines, balanced output)'"
complete -c batless -l custom-profile -d 'Load custom AI profile from file' -r
complete -c batless -l config -d 'Configuration file path (defaults to auto-discovery)' -r
complete -c batless -l streaming-chunk-size -d 'Chunk size for streaming output (in lines)' -r
complete -c batless -l checkpoint -d 'Checkpoint file path for resuming' -r
complete -c batless -l edit-profile -d 'Edit existing custom profile' -r
complete -c batless -l strip-ansi -d 'Strip ANSI escape codes from output'
complete -c batless -l list-languages -d 'List all supported languages'
complete -c batless -l list-themes -d 'List all available themes'
complete -c batless -l include-tokens -d 'Include tokens in JSON output (AI-friendly)'
complete -c batless -l summary -d 'Summary mode: show only important code structures (deprecated, use --summary-level)'
complete -c batless -l count-tokens -d 'Count tokens for AI model context estimation'
complete -c batless -l fit-context -d 'Fit content within AI model context window (truncate if needed)'
complete -c batless -l validate-json -d 'Validate JSON output against schema'
complete -c batless -l streaming-json -d 'Enable streaming JSON output for large files'
complete -c batless -l enable-resume -d 'Enable resume capability with checkpoint support'
complete -c batless -l configure -d 'Run interactive configuration wizard'
complete -c batless -l list-profiles -d 'List available custom profiles'
complete -c batless -l debug -d 'Enable debug mode with detailed processing information'
complete -c batless -l plain -d 'PAGER compatibility: equivalent to --mode plain (for cat replacement)'
complete -c batless -s u -l unbuffered -d 'PAGER compatibility: ignored for compatibility with other pagers'
complete -c batless -s n -l number -d 'CAT compatibility: show line numbers (like cat -n)'
complete -c batless -s b -l number-nonblank -d 'CAT compatibility: number non-blank output lines (like cat -b)'
complete -c batless -l no-title -d 'PAGER compatibility: ignored for compatibility with less (no title bar)'
complete -c batless -l version-json -d 'Output version information as machine-readable JSON'
complete -c batless -l json-pretty -d 'Pretty-print JSON output (when --mode=json); does not affect streaming'
complete -c batless -s h -l help -d 'Print help (see more with \'--help\')'
complete -c batless -s V -l version -d 'Print version'
