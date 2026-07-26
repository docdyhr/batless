
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
            [CompletionResult]::new('--generate-completions', '--generate-completions', [CompletionResultType]::ParameterName, 'Generate shell completions for the specified shell')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Configuration file path (defaults to auto-discovery)')
            [CompletionResult]::new('--strip-ansi', '--strip-ansi', [CompletionResultType]::ParameterName, 'Strip ANSI escape codes from output')
            [CompletionResult]::new('--list-languages', '--list-languages', [CompletionResultType]::ParameterName, 'List all supported languages')
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
            [CompletionResult]::new('--json-pretty', '--json-pretty', [CompletionResultType]::ParameterName, 'Pretty-print JSON output (when --mode=json)')
            [CompletionResult]::new('--with-line-numbers', '--with-line-numbers', [CompletionResultType]::ParameterName, 'Include 1-based line numbers in JSON output lines array (e.g. {"n":1,"text":"..."})')
            [CompletionResult]::new('--strip-comments', '--strip-comments', [CompletionResultType]::ParameterName, 'Strip comment-only lines from output')
            [CompletionResult]::new('--strip-blank-lines', '--strip-blank-lines', [CompletionResultType]::ParameterName, 'Strip blank lines from output')
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
