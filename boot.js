// boot.js
// types out a fake boot log in #boot, then reveals #content.
// skips straight to the end state for reduced-motion or repeat visits
// in the same session, so it's a one-time thing, not a tax on every load.

(function () {
  var lines = [
    "lunix-boot: starting as pid 1",
    "lunix-boot: mount /proc ... ok",
    "lunix-boot: mount /sys ... ok",
    "lunix-boot: mount /dev ... ok",
    "lunix-boot: exec lunix-core",
    "lunix-core: scanning ~/.lunix/modules ...",
    "lunix-core: ready"
  ];

  var bootEl = document.getElementById("boot");
  var contentEl = document.getElementById("content");
  if (!bootEl || !contentEl) return;

  var reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  var seen = false;
  try { seen = sessionStorage.getItem("lunix-booted") === "1"; } catch (e) {}

  if (reduceMotion || seen) {
    bootEl.classList.add("hidden");
    contentEl.classList.remove("hidden");
    return;
  }

  try { sessionStorage.setItem("lunix-booted", "1"); } catch (e) {}

  contentEl.classList.add("hidden");
  bootEl.innerHTML = "";

  var lineIndex = 0;
  var charIndex = 0;
  var current = document.createElement("div");
  bootEl.appendChild(current);

  function tick() {
    if (lineIndex >= lines.length) {
      var cursor = document.createElement("span");
      cursor.className = "cursor";
      cursor.innerHTML = "&nbsp;";
      current.appendChild(cursor);
      setTimeout(function () {
        bootEl.classList.add("hidden");
        contentEl.classList.remove("hidden");
      }, 350);
      return;
    }

    var line = lines[lineIndex];
    if (charIndex <= line.length) {
      var text = line.slice(0, charIndex);
      if (line.indexOf("... ok") !== -1 && charIndex >= line.length) {
        text = line.replace("... ok", '... <span class="ok">ok</span>');
        current.innerHTML = text;
      } else {
        current.textContent = text;
      }
      charIndex++;
      setTimeout(tick, 12);
    } else {
      lineIndex++;
      charIndex = 0;
      current = document.createElement("div");
      bootEl.appendChild(current);
      setTimeout(tick, 90);
    }
  }

  tick();
})();
