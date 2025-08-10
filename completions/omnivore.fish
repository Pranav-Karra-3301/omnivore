# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_omnivore_global_optspecs
	string join \n v/verbose c/config= h/help V/version
end

function __fish_omnivore_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_omnivore_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_omnivore_using_subcommand
	set -l cmd (__fish_omnivore_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c omnivore -n "__fish_omnivore_needs_command" -s c -l config -r -F
complete -c omnivore -n "__fish_omnivore_needs_command" -s v -l verbose
complete -c omnivore -n "__fish_omnivore_needs_command" -s h -l help -d 'Print help'
complete -c omnivore -n "__fish_omnivore_needs_command" -s V -l version -d 'Print version'
complete -c omnivore -n "__fish_omnivore_needs_command" -f -a "crawl"
complete -c omnivore -n "__fish_omnivore_needs_command" -f -a "parse"
complete -c omnivore -n "__fish_omnivore_needs_command" -f -a "graph"
complete -c omnivore -n "__fish_omnivore_needs_command" -f -a "stats"
complete -c omnivore -n "__fish_omnivore_needs_command" -f -a "git"
complete -c omnivore -n "__fish_omnivore_needs_command" -f -a "generate-completions"
complete -c omnivore -n "__fish_omnivore_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c omnivore -n "__fish_omnivore_using_subcommand crawl" -s w -l workers -r
complete -c omnivore -n "__fish_omnivore_using_subcommand crawl" -s d -l depth -r
complete -c omnivore -n "__fish_omnivore_using_subcommand crawl" -s o -l output -d 'Output file for results' -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand crawl" -l delay -d 'Delay between requests in milliseconds' -r
complete -c omnivore -n "__fish_omnivore_using_subcommand crawl" -s c -l config -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand crawl" -l respect-robots -d 'Respect robots.txt'
complete -c omnivore -n "__fish_omnivore_using_subcommand crawl" -s v -l verbose
complete -c omnivore -n "__fish_omnivore_using_subcommand crawl" -s h -l help -d 'Print help'
complete -c omnivore -n "__fish_omnivore_using_subcommand parse" -s r -l rules -d 'Parsing rules file' -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand parse" -s c -l config -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand parse" -s v -l verbose
complete -c omnivore -n "__fish_omnivore_using_subcommand parse" -s h -l help -d 'Print help'
complete -c omnivore -n "__fish_omnivore_using_subcommand graph" -s o -l output -d 'Output graph file' -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand graph" -s c -l config -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand graph" -s v -l verbose
complete -c omnivore -n "__fish_omnivore_using_subcommand graph" -s h -l help -d 'Print help'
complete -c omnivore -n "__fish_omnivore_using_subcommand stats" -s c -l config -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand stats" -s v -l verbose
complete -c omnivore -n "__fish_omnivore_using_subcommand stats" -s h -l help -d 'Print help'
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l include -d 'Include only files matching these patterns (comma-separated)' -r
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l exclude -d 'Exclude files matching these patterns (comma-separated)' -r
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l output -d 'Output filtered files to directory' -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l depth -d 'Clone depth for remote repositories' -r
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l max-file-size -d 'Maximum file size in bytes (e.g., 10485760 for 10MB)' -r
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -s c -l config -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l no-gitignore -d 'Ignore .gitignore files'
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l json -d 'Output as JSON'
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l txt -d 'Output as plain text'
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l keep -d 'Keep temporary clone after completion (for remote repos)'
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l allow-binary -d 'Include binary files in output'
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -l verbose -d 'Verbose output'
complete -c omnivore -n "__fish_omnivore_using_subcommand git" -s h -l help -d 'Print help'
complete -c omnivore -n "__fish_omnivore_using_subcommand generate-completions" -s c -l config -r -F
complete -c omnivore -n "__fish_omnivore_using_subcommand generate-completions" -s v -l verbose
complete -c omnivore -n "__fish_omnivore_using_subcommand generate-completions" -s h -l help -d 'Print help'
complete -c omnivore -n "__fish_omnivore_using_subcommand help; and not __fish_seen_subcommand_from crawl parse graph stats git generate-completions help" -f -a "crawl"
complete -c omnivore -n "__fish_omnivore_using_subcommand help; and not __fish_seen_subcommand_from crawl parse graph stats git generate-completions help" -f -a "parse"
complete -c omnivore -n "__fish_omnivore_using_subcommand help; and not __fish_seen_subcommand_from crawl parse graph stats git generate-completions help" -f -a "graph"
complete -c omnivore -n "__fish_omnivore_using_subcommand help; and not __fish_seen_subcommand_from crawl parse graph stats git generate-completions help" -f -a "stats"
complete -c omnivore -n "__fish_omnivore_using_subcommand help; and not __fish_seen_subcommand_from crawl parse graph stats git generate-completions help" -f -a "git"
complete -c omnivore -n "__fish_omnivore_using_subcommand help; and not __fish_seen_subcommand_from crawl parse graph stats git generate-completions help" -f -a "generate-completions"
complete -c omnivore -n "__fish_omnivore_using_subcommand help; and not __fish_seen_subcommand_from crawl parse graph stats git generate-completions help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
