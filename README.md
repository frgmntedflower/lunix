# lunix site

plain html/css/js. no build step, no framework, no node_modules.
open index.html in a browser and it works.

```
lunix-site/
    index.html          landing page
    styles.css           all styles, one file
    boot.js               the boot-log typing intro
    docs/
        index.html         docs hub
        architecture.html
        module-contract.html
        languages.html
```

## before you push this live

search for `YOURNAME` across the html files and replace with your
actual github username/repo path. every github link is a placeholder
right now.

```sh
grep -rl YOURNAME . | xargs sed -i 's/YOURNAME/actual-username/g'
```

## deploying to github pages

simplest path: put this folder's contents at the root of a repo
(either the main lunix repo on a `gh-pages` branch, or a separate
repo, your call), then in that repo's settings turn on pages,
pointing at the branch/root. no build step means no github actions
workflow needed, pages serves the static files directly.

if you'd rather keep it inside the main lunix repo without a
separate branch, github pages also supports serving from a `/docs`
folder on `main`. that would mean renaming this whole folder to
`docs/` at the repo root instead of the markdown docs living there
now, so pick one, don't mix rendered html and source markdown docs
in the same folder path.

## adding a module's docs later

when a module ships, add a page under `docs/`, matching the style of
the existing ones, and link it from the relevant category list in
`docs/index.html`. no other page needs to change.
