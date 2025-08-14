
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'batless' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'batless'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'batless' {
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'Language for syntax highlighting (auto-detect if not specified)')
            [CompletionResult]::new('--max-lines', '--max-lines', [CompletionResultType]::ParameterName, 'Limit lines shown')
            [CompletionResult]::new('--max-bytes', '--max-bytes', [CompletionResultType]::ParameterName, 'Limit bytes shown')
            [CompletionResult]::new('--mode', '--mode', [CompletionResultType]::ParameterName, 'Output mode')
            [CompletionResult]::new('--color', '--color', [CompletionResultType]::ParameterName, 'Color output control')
            [CompletionResult]::new('--theme', '--theme', [CompletionResultType]::ParameterName, 'Theme for syntax highlighting')
            [CompletionResult]::new('--summary-level', '--summary-level', [CompletionResultType]::ParameterName, 'Summary level: control detail level of summary output')
            [CompletionResult]::new('--ai-model', '--ai-model', [CompletionResultType]::ParameterName, 'AI model for token counting')
            [CompletionResult]::new('--prompt-tokens', '--prompt-tokens', [CompletionResultType]::ParameterName, 'Estimate prompt token overhead when fitting context')
            [CompletionResult]::new('--get-schema', '--get-schema', [CompletionResultType]::ParameterName, 'Get JSON schema for specified output format')
            [CompletionResult]::new('--generate-completions', '--generate-completions', [CompletionResultType]::ParameterName, 'Generate shell completions for the specified shell')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use predefined AI tool profile (overrides other settings)')
            [CompletionResult]::new('--custom-profile', '--custom-profile', [CompletionResultType]::ParameterName, 'Load custom AI profile from file')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Configuration file path (defaults to auto-discovery)')
            [CompletionResult]::new('--streaming-chunk-size', '--streaming-chunk-size', [CompletionResultType]::ParameterName, 'Chunk size for streaming output (in lines)')
            [CompletionResult]::new('--checkpoint', '--checkpoint', [CompletionResultType]::ParameterName, 'Checkpoint file path for resuming')
            [CompletionResult]::new('--edit-profile', '--edit-profile', [CompletionResultType]::ParameterName, 'Edit existing custom profile')
            [CompletionResult]::new('--strip-ansi', '--strip-ansi', [CompletionResultType]::ParameterName, 'Strip ANSI escape codes from output')
            [CompletionResult]::new('--list-languages', '--list-languages', [CompletionResultType]::ParameterName, 'List all supported languages')
            [CompletionResult]::new('--list-themes', '--list-themes', [CompletionResultType]::ParameterName, 'List all available themes')
            [CompletionResult]::new('--include-tokens', '--include-tokens', [CompletionResultType]::ParameterName, 'Include tokens in JSON output (AI-friendly)')
            [CompletionResult]::new('--summary', '--summary', [CompletionResultType]::ParameterName, 'Summary mode: show only important code structures (deprecated, use --summary-level)')
            [CompletionResult]::new('--count-tokens', '--count-tokens', [CompletionResultType]::ParameterName, 'Count tokens for AI model context estimation')
            [CompletionResult]::new('--fit-context', '--fit-context', [CompletionResultType]::ParameterName, 'Fit content within AI model context window (truncate if needed)')
            [CompletionResult]::new('--validate-json', '--validate-json', [CompletionResultType]::ParameterName, 'Validate JSON output against schema')
            [CompletionResult]::new('--streaming-json', '--streaming-json', [CompletionResultType]::ParameterName, 'Enable streaming JSON output for large files')
            [CompletionResult]::new('--enable-resume', '--enable-resume', [CompletionResultType]::ParameterName, 'Enable resume capability with checkpoint support')
            [CompletionResult]::new('--configure', '--configure', [CompletionResultType]::ParameterName, 'Run interactive configuration wizard')
            [CompletionResult]::new('--list-profiles', '--list-profiles', [CompletionResultType]::ParameterName, 'List available custom profiles')
            [CompletionResult]::new('--debug', '--debug', [CompletionResultType]::ParameterName, 'Enable debug mode with detailed processing information')
            [CompletionResult]::new('--plain', '--plain', [CompletionResultType]::ParameterName, 'PAGER compatibility: equivalent to --mode plain (for cat replacement)')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'PAGER compatibility: ignored for compatibility with other pagers')
            [CompletionResult]::new('--unbuffered', '--unbuffered', [CompletionResultType]::ParameterName, 'PAGER compatibility: ignored for compatibility with other pagers')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'CAT compatibility: show line numbers (like cat -n)')
            [CompletionResult]::new('--number', '--number', [CompletionResultType]::ParameterName, 'CAT compatibility: show line numbers (like cat -n)')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'CAT compatibility: number non-blank output lines (like cat -b)')
            [CompletionResult]::new('--number-nonblank', '--number-nonblank', [CompletionResultType]::ParameterName, 'CAT compatibility: number non-blank output lines (like cat -b)')
            [CompletionResult]::new('--no-title', '--no-title', [CompletionResultType]::ParameterName, 'PAGER compatibility: ignored for compatibility with less (no title bar)')
            [CompletionResult]::new('--version-json', '--version-json', [CompletionResultType]::ParameterName, 'Output version information as machine-readable JSON')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
