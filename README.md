# Nuzlocke Generator Engine

> What is this ?

If you are an amateur of Pokemon nuzlocke runs, you might have faced this issue
where hard games such as Run&Bun and Emerald Kaizo are fun but too hard to play
and other games are either too easy, not well documented, or lacking QoL features
suited for a nuzlocke challenge. There is a lack of intermediate difficulty games
that are designed to be nuzlocked.

This repository is for now just me toying with the idea of a pokemon randomizer
that would be able to generate interesting nuzlocke runs, based on decomp projects
instead of directly based on rom files.

## Planned features & long term vision

The goal of this project is to provide an engine that, given a decompilation project
along with a "bundle" of configurations (such as a list of pokemon set, possible wild
encounters, etc), can generate a game that is interesting to nuzlocke, With fully
detailed documentation.

Ideally, the engine will be designed to have a unique intermediate representation
of everything needed to generate a game, and compatibility layers for decomp projects.
The idea behind this is to have any bundle compatible with any decomp project,
given the apropriate compatibility layer.

For now the project will be focused on being compatible with the pokeemerald-expansion
project from the RHHideout.

Key parts of the engine could be exposed to scripting languages for easy tweaking of
the generation/randomization process.
