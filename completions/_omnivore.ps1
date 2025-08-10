
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'omnivore' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'omnivore'
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
        'omnivore' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'config')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'verbose')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('crawl', 'crawl', [CompletionResultType]::ParameterValue, 'crawl')
            [CompletionResult]::new('parse', 'parse', [CompletionResultType]::ParameterValue, 'parse')
            [CompletionResult]::new('graph', 'graph', [CompletionResultType]::ParameterValue, 'graph')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'stats')
            [CompletionResult]::new('git', 'git', [CompletionResultType]::ParameterValue, 'git')
            [CompletionResult]::new('generate-completions', 'generate-completions', [CompletionResultType]::ParameterValue, 'generate-completions')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'omnivore;crawl' {
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'w')
            [CompletionResult]::new('--workers', '--workers', [CompletionResultType]::ParameterName, 'workers')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--depth', '--depth', [CompletionResultType]::ParameterName, 'depth')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output file for results')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output file for results')
            [CompletionResult]::new('--delay', '--delay', [CompletionResultType]::ParameterName, 'Delay between requests in milliseconds')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'config')
            [CompletionResult]::new('--respect-robots', '--respect-robots', [CompletionResultType]::ParameterName, 'Respect robots.txt')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'verbose')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'omnivore;parse' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Parsing rules file')
            [CompletionResult]::new('--rules', '--rules', [CompletionResultType]::ParameterName, 'Parsing rules file')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'config')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'verbose')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'omnivore;graph' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output graph file')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output graph file')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'config')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'verbose')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'omnivore;stats' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'config')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'verbose')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'omnivore;git' {
            [CompletionResult]::new('--include', '--include', [CompletionResultType]::ParameterName, 'Include only files matching these patterns (comma-separated)')
            [CompletionResult]::new('--exclude', '--exclude', [CompletionResultType]::ParameterName, 'Exclude files matching these patterns (comma-separated)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output filtered files to directory')
            [CompletionResult]::new('--depth', '--depth', [CompletionResultType]::ParameterName, 'Clone depth for remote repositories')
            [CompletionResult]::new('--max-file-size', '--max-file-size', [CompletionResultType]::ParameterName, 'Maximum file size in bytes (e.g., 10485760 for 10MB)')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'config')
            [CompletionResult]::new('--no-gitignore', '--no-gitignore', [CompletionResultType]::ParameterName, 'Ignore .gitignore files')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'Output as JSON')
            [CompletionResult]::new('--txt', '--txt', [CompletionResultType]::ParameterName, 'Output as plain text')
            [CompletionResult]::new('--keep', '--keep', [CompletionResultType]::ParameterName, 'Keep temporary clone after completion (for remote repos)')
            [CompletionResult]::new('--allow-binary', '--allow-binary', [CompletionResultType]::ParameterName, 'Include binary files in output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Verbose output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'omnivore;generate-completions' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'config')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'verbose')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'omnivore;help' {
            [CompletionResult]::new('crawl', 'crawl', [CompletionResultType]::ParameterValue, 'crawl')
            [CompletionResult]::new('parse', 'parse', [CompletionResultType]::ParameterValue, 'parse')
            [CompletionResult]::new('graph', 'graph', [CompletionResultType]::ParameterValue, 'graph')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'stats')
            [CompletionResult]::new('git', 'git', [CompletionResultType]::ParameterValue, 'git')
            [CompletionResult]::new('generate-completions', 'generate-completions', [CompletionResultType]::ParameterValue, 'generate-completions')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'omnivore;help;crawl' {
            break
        }
        'omnivore;help;parse' {
            break
        }
        'omnivore;help;graph' {
            break
        }
        'omnivore;help;stats' {
            break
        }
        'omnivore;help;git' {
            break
        }
        'omnivore;help;generate-completions' {
            break
        }
        'omnivore;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
