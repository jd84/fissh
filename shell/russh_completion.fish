function __fish_russh_complete_hosts
    russh -c /home/jan/.ssh/russh.yml --list -f none
end

complete -c russh -n "not __fish_seen_subcommand_from (__fish_russh_complete_hosts)" -a "(__fish_russh_complete_hosts)"
