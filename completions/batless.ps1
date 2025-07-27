
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
            [CompletionResult]::new('--generate-completions', '--generate-completions', [CompletionResultType]::ParameterName, 'Generate shell completions for the specified shell')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use predefined AI tool profile (overrides other settings)')
            [CompletionResult]::new('--strip-ansi', '--strip-ansi', [CompletionResultType]::ParameterName, 'Strip ANSI escape codes from output')
            [CompletionResult]::new('--list-languages', '--list-languages', [CompletionResultType]::ParameterName, 'List all supported languages')
            [CompletionResult]::new('--list-themes', '--list-themes', [CompletionResultType]::ParameterName, 'List all available themes')
            [CompletionResult]::new('--include-tokens', '--include-tokens', [CompletionResultType]::ParameterName, 'Include tokens in JSON output (AI-friendly)')
            [CompletionResult]::new('--summary', '--summary', [CompletionResultType]::ParameterName, 'Summary mode: show only important code structures')
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
