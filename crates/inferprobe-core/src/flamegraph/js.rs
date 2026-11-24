//! Client-side JavaScript for interactive SVG flamegraph controls.

/// Returns embedded JavaScript for interactive flamegraph zooming and search filtering.
pub fn get_report_js() -> &'static str {
    r#"
let currentScale = 1.0;

function filterSpans(query) {
  const q = query.trim().toLowerCase();
  const nodes = document.querySelectorAll('#flame-nodes .flame-rect');
  nodes.forEach(node => {
    if (!q) {
      node.classList.remove('dimmed');
      return;
    }
    const name = (node.getAttribute('data-name') || '').toLowerCase();
    const stage = (node.getAttribute('data-stage') || '').toLowerCase();
    const dev = (node.getAttribute('data-device') || '').toLowerCase();
    if (name.includes(q) || stage.includes(q) || dev.includes(q)) {
      node.classList.remove('dimmed');
    } else {
      node.classList.add('dimmed');
    }
  });
}

function zoomFlame(factor) {
  currentScale = Math.max(0.2, Math.min(10.0, currentScale * factor));
  const svg = document.getElementById('flamegraph-svg');
  if (svg) {
    svg.style.width = (currentScale * 100) + '%';
  }
}

function resetFlame() {
  currentScale = 1.0;
  const svg = document.getElementById('flamegraph-svg');
  if (svg) {
    svg.style.width = '100%';
  }
  const searchInput = document.getElementById('flame-search');
  if (searchInput) {
    searchInput.value = '';
    filterSpans('');
  }
  const details = document.getElementById('flame-details');
  if (details) {
    details.textContent = 'Hover or click a span to inspect trace attributes';
  }
  document.querySelectorAll('#flame-nodes .flame-rect').forEach(node => {
    node.classList.remove('selected');
    node.classList.remove('dimmed');
  });
}

function selectSpan(elem) {
  document.querySelectorAll('#flame-nodes .flame-rect').forEach(node => node.classList.remove('selected'));
  elem.classList.add('selected');
  const name = elem.getAttribute('data-name');
  const stage = elem.getAttribute('data-stage');
  const dev = elem.getAttribute('data-device');
  const start = elem.getAttribute('data-start');
  const dur = elem.getAttribute('data-dur');
  const rss = elem.getAttribute('data-rss');
  const details = document.getElementById('flame-details');
  if (details) {
    details.innerHTML = '<strong>' + name + '</strong> [' + stage + ' | ' + dev + '] &mdash; Start: ' + start + ' us, Duration: ' + dur + ' us, RSS: ' + rss + ' KB';
  }
}
"#
}
