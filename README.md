# Svelte(rs)

A WIP parser for svelte files that is designed with error recovery and reporting in mind. The intention is to be able to use it in editor tooling. There is no intention to rewrite the svelte compiler in rust, though a project attempting
to do that could use this parser as a starting point.

This is very work-in-progress and can't parse most svelte code right now (it can't even parse
elements yet!).
