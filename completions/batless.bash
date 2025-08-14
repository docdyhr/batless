_batless() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="batless"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        batless)
            opts="-u -n -b -h -V --language --max-lines --max-bytes --mode --color --theme --strip-ansi --list-languages --list-themes --include-tokens --summary --summary-level --count-tokens --ai-model --fit-context --prompt-tokens --validate-json --get-schema --generate-completions --profile --custom-profile --config --streaming-json --streaming-chunk-size --enable-resume --checkpoint --configure --list-profiles --edit-profile --debug --plain --unbuffered --number --number-nonblank --no-title --version-json --help --version [FILE]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --language)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-lines)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-bytes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --mode)
                    COMPREPLY=($(compgen -W "plain highlight json summary" -- "${cur}"))
                    return 0
                    ;;
                --color)
                    COMPREPLY=($(compgen -W "auto always never" -- "${cur}"))
                    return 0
                    ;;
                --theme)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --summary-level)
                    COMPREPLY=($(compgen -W "none minimal standard detailed" -- "${cur}"))
                    return 0
                    ;;
                --ai-model)
                    COMPREPLY=($(compgen -W "gpt4 gpt4-turbo gpt35 claude claude35-sonnet generic" -- "${cur}"))
                    return 0
                    ;;
                --prompt-tokens)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --get-schema)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --generate-completions)
                    COMPREPLY=($(compgen -W "bash zsh fish power-shell" -- "${cur}"))
                    return 0
                    ;;
                --profile)
                    COMPREPLY=($(compgen -W "claude copilot chatgpt assistant" -- "${cur}"))
                    return 0
                    ;;
                --custom-profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --streaming-chunk-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --checkpoint)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --edit-profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _batless -o nosort -o bashdefault -o default batless
else
    complete -F _batless -o bashdefault -o default batless
fi
