function __fish_russh_complete_hosts
    russh --list -f none | awk -F '[] []' '{print $0}'
end

complete -f -c russh -n "not __fish_seen_subcommand_from (__fish_russh_complete_hosts)" -a "(__fish_russh_complete_hosts)"
