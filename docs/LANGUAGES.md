# languages

decided once here so it doesn't get relitigated every module.

boot + core: rust. pid 1 is the one place a bug takes the whole
machine down with it, memory safety matters more here than anywhere
else, and rust gives that without a garbage collector, so it's still
a real fit for init and launcher work.

modules, default: c. matches dwm, st, dmenu, the kernel itself, every
real init that came before this one. text editor, RE tool, anything
that should feel at home next to suckless tools, it's c unless
there's a specific reason otherwise.

networking / security modules: go. wrong fit for boot, wrong fit for
most modules, right fit here. static binaries, fast to write, easy
to ship and cross compile.

asm: inside modules, never as a whole module. hot loop in a
renderer, a perf critical path. spice, not the meal.

the manifest doesn't care what built the binary, lunix.toml just
points at an executable. this file is a style decision for
consistency, not something enforced in code.
